// Copyright 2022 Webb Technologies Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{handlers::validate_proposals::ValidationError, ProposalNonce};
use codec::{alloc::string::ToString, Decode};
use ethereum::{
	EIP1559TransactionMessage, EIP2930TransactionMessage, LegacyTransactionMessage, TransactionV2,
};

pub struct EvmTxProposal {
	pub chain_id: u32,
	pub nonce: ProposalNonce,
	pub tx: TransactionV2,
}

/// https://github.com/webb-tools/protocol-solidity/issues/83
/// Proposal Data: [
///     bytes,
///     variable [0..]
/// ]
pub fn create(data: &[u8]) -> Result<EvmTxProposal, ValidationError> {
	let eth_transaction = TransactionV2::decode(&mut &*data)
		.map_err(|_| ValidationError::InvalidParameter("Invalid transaction".to_string()))?;

	if !validate_ethereum_tx(&eth_transaction) {
		return Err(ValidationError::InvalidParameter("Ethereum transaction is invalid".to_string()))
	}

	let (chain_id, nonce) = decode_evm_transaction(&eth_transaction)?;

	// TODO: Add validation over EVM address
	Ok(EvmTxProposal { chain_id, nonce, tx: eth_transaction })
}

fn decode_evm_transaction(
	eth_transaction: &TransactionV2,
) -> Result<(u32, ProposalNonce), ValidationError> {
	let (chain_id, nonce) = match eth_transaction {
		TransactionV2::Legacy(tx) => {
			let chain_id: u32 = 0;
			let nonce = tx.nonce.as_u32();
			(chain_id, nonce)
		},
		TransactionV2::EIP2930(tx) => {
			let chain_id: u32 = tx.chain_id.try_into().map_err(|_| {
				ValidationError::InvalidParameter(
					"Invalid chain id (could not fit into u32)".to_string(),
				)
			})?;
			let nonce = tx.nonce.as_u32();
			(chain_id, nonce)
		},
		TransactionV2::EIP1559(tx) => {
			let chain_id: u32 = tx.chain_id.try_into().map_err(|_| {
				ValidationError::InvalidParameter(
					"Invalid chain id (could not fit into u32)".to_string(),
				)
			})?;
			let nonce = tx.nonce.as_u32();
			(chain_id, nonce)
		},
	};

	Ok((chain_id, nonce.into()))
}

fn validate_ethereum_tx(eth_transaction: &TransactionV2) -> bool {
	match eth_transaction {
		TransactionV2::Legacy(_tx) => true,
		TransactionV2::EIP2930(_tx) => true,
		TransactionV2::EIP1559(_tx) => true,
	}
}

#[allow(dead_code)]
fn validate_ethereum_tx_signature(eth_transaction: &TransactionV2) -> bool {
	let (sig_r, sig_s, sig_v, msg_hash) = match eth_transaction {
		TransactionV2::Legacy(tx) => {
			let r = *tx.signature.r();
			let s = *tx.signature.s();
			let v = tx.signature.standard_v();
			let hash = LegacyTransactionMessage::from(tx.clone()).hash();
			(r, s, v, hash)
		},
		TransactionV2::EIP2930(tx) => {
			let r = tx.r;
			let s = tx.s;
			let v = if tx.odd_y_parity { 1 } else { 0 };
			let hash = EIP2930TransactionMessage::from(tx.clone()).hash();
			(r, s, v, hash)
		},
		TransactionV2::EIP1559(tx) => {
			let r = tx.r;
			let s = tx.s;
			let v = if tx.odd_y_parity { 1 } else { 0 };
			let hash = EIP1559TransactionMessage::from(tx.clone()).hash();
			(r, s, v, hash)
		},
	};

	let mut sig = [0u8; 65];
	sig[0..32].copy_from_slice(&sig_r[..]);
	sig[32..64].copy_from_slice(&sig_s[..]);
	sig[64] = sig_v;
	let mut msg = [0u8; 32];
	msg.copy_from_slice(&msg_hash[..]);

	sp_io::crypto::secp256k1_ecdsa_recover(&sig, &msg).is_ok()
}
