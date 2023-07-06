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

use crate::{
	debug_logger::DebugLogger, storage::proposals::generate_delayed_submit_at,
	worker::MAX_SUBMISSION_DELAY,
};
use codec::Encode;
use dkg_primitives::types::{DKGError, SessionId};
use dkg_runtime_primitives::{
	offchain::storage_keys::{
		AGGREGATED_PUBLIC_KEYS, AGGREGATED_PUBLIC_KEYS_AT_GENESIS, SUBMIT_GENESIS_KEYS_AT,
		SUBMIT_KEYS_AT,
	},
	AggregatedPublicKeys,
};
use sc_client_api::Backend;
use sp_api::offchain::{OffchainStorage, STORAGE_PREFIX};
use sp_runtime::traits::{Block, NumberFor};
use std::{collections::HashMap, sync::Arc};

/// stores genesis or next aggregated public keys offchain
pub(crate) fn store_aggregated_public_keys<B, BE>(
	backend: &Arc<BE>,
	aggregated_public_keys: &mut HashMap<SessionId, AggregatedPublicKeys>,
	is_genesis_round: bool,
	session_id: SessionId,
	current_block_number: NumberFor<B>,
	logger: &DebugLogger,
) -> Result<(), DKGError>
where
	B: Block,
	BE: Backend<B>,
{
	let maybe_offchain = backend.offchain_storage();
	if maybe_offchain.is_none() {
		return Err(DKGError::GenericError { reason: "No offchain storage available".to_string() })
	}
	let offchain = maybe_offchain.expect("checked above");
	let keys = aggregated_public_keys.get(&session_id).ok_or_else(|| DKGError::CriticalError {
		reason: format!("Aggregated public key for session {session_id} does not exist in map"),
	})?;
	if is_genesis_round {
		//dkg_worker.dkg_state.listening_for_active_pub_key = false;
		perform_storing_of_aggregated_public_keys::<B, BE>(
			offchain,
			keys,
			current_block_number,
			AGGREGATED_PUBLIC_KEYS_AT_GENESIS,
			SUBMIT_GENESIS_KEYS_AT,
			logger,
		);
	} else {
		//dkg_worker.dkg_state.listening_for_pub_key = false;
		perform_storing_of_aggregated_public_keys::<B, BE>(
			offchain,
			keys,
			current_block_number,
			AGGREGATED_PUBLIC_KEYS,
			SUBMIT_KEYS_AT,
			logger,
		);
		let _ = aggregated_public_keys.remove(&session_id);
	}

	Ok(())
}

/// stores the aggregated public keys
pub fn perform_storing_of_aggregated_public_keys<B, BE>(
	mut offchain: <BE as Backend<B>>::OffchainStorage,
	keys: &AggregatedPublicKeys,
	current_block_number: NumberFor<B>,
	aggregated_keys: &[u8],
	submit_keys: &[u8],
	logger: &DebugLogger,
) where
	B: Block,
	BE: Backend<B>,
{
	offchain.set(STORAGE_PREFIX, aggregated_keys, &keys.encode());
	let submit_at = generate_delayed_submit_at::<B>(current_block_number, MAX_SUBMISSION_DELAY);
	if let Some(submit_at) = submit_at {
		logger.debug(format!("Storing aggregated public keys at block {submit_at}"));
		offchain.set(STORAGE_PREFIX, submit_keys, &submit_at.encode());
	}
}
