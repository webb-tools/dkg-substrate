use codec::Encode;
use curv::{arithmetic::Converter, elliptic::curves::Secp256k1, BigInt};
use log::{debug, error, info, trace, warn};
use round_based::{IsCritical, Msg, StateMachine};
use sc_keystore::LocalKeystore;
use sp_core::{ecdsa::Signature, sr25519, Pair as TraitPair};
use sp_runtime::traits::AtLeast32BitUnsigned;
use std::{
	collections::{BTreeMap, HashMap},
	path::PathBuf,
	sync::Arc,
};

use crate::{
	types::*,
	utils::{select_random_set, store_localkey, vec_usize_to_u16},
};
use dkg_runtime_primitives::{
	keccak_256,
	offchain_crypto::{Pair as AppPair, Public},
};

pub use gg_2020::{
	party_i::*,
	state_machine::{keygen::*, sign::*},
};
pub use multi_party_ecdsa::protocols::multi_party_ecdsa::{
	gg_2020,
	gg_2020::state_machine::{keygen as gg20_keygen, sign as gg20_sign, traits::RoundBlame},
};

pub enum OfflineState<C> {
	NotStarted(PreOfflineRounds<C>),
	Started(OfflineRounds<C>),
	Finished(Result<CompletedOfflineStage, DKGError>),
}

/// Pre-offline rounds

pub struct PreOfflineRounds<Clock> {
	signer_set_id: SignerSetId,
	pending_offline_msgs: Vec<DKGOfflineMessage>,
}

impl<C> PreOfflineRounds<C>
where
	C: AtLeast32BitUnsigned + Copy,
{
	pub fn new(signer_set_id: SignerSetId) -> Self {
		Self{
			signer_set_id,
			pending_keygen_msgs: Vec::default(),
		}
	}
}

impl<C> DKGRoundsSM<DKGOfflineMessage, Vec<DKGOfflineMessage>, C> for PreOfflineRounds<C>
where
	C: AtLeast32BitUnsigned + Copy,
{
	pub fn handle_incoming(&mut self, data: DKGOfflineMessage) -> Result<(), DKGError> {
		self.pending_offline_msgs.push(data);
		Ok(())
	}

	pub fn is_finished(&self) {
		true
	}
	
	pub fn try_finish(self) -> Result<Vec<DKGOfflineMessage>, DKGError> {
		Ok(self.pending_offline_msgs.take())
	}
}

/// Offline rounds

pub struct OfflineRounds<Clock> {
	round_id: RoundId,
	party_index: u16,
	threshold: u16,
	parties: u16,

	signer_set_id: SignerSetId,
	signers: Vec<u16>,

	// DKG clock
	offline_started_at: Clock,
	// The block number at which a dkg message was last received
	last_received_at: Clock,

	// Message processing
	pending_offline_msgs: HashMap<Vec<u8>, Vec<DKGOfflineMessage>>,

	// Offline stage
	offline_stage: OfflineStage,
}

impl<C> OfflineRounds<C>
where
	C: AtLeast32BitUnsigned + Copy,
{
	/// Proceed to next step for current Stage

	fn proceed_offline_stage(&mut self, key: Vec<u8>, at: C) -> Result<bool, DKGError> {
		trace!(target: "dkg", "🕸️  OfflineStage party {} enter proceed", self.party_index);

		if !self.offline_stage.contains_key(&key) {
			return Ok(false)
		}

		let offline_stage = self.offline_stage.get_mut(&key).unwrap();

		if offline_stage.wants_to_proceed() {
			info!(target: "dkg", "🕸️  OfflineStage party {} wants to proceed", offline_stage.party_ind());
			trace!(target: "dkg", "🕸️  before: {:?}", offline_stage);
			// TODO, handle asynchronously
			match offline_stage.proceed() {
				Ok(_) => {
					trace!(target: "dkg", "🕸️  after: {:?}", offline_stage);
				},
				Err(err) => {
					println!("Error proceeding offline stage {:?}", err);
					match err {
						gg20_sign::Error::ProceedRound(proceed_err) => match proceed_err {
							gg20_sign::rounds::Error::Round1(err_type) =>
								return Err(DKGError::OfflineMisbehaviour {
									bad_actors: vec_usize_to_u16(err_type.bad_actors),
								}),
							gg20_sign::rounds::Error::Round2Stage4(err_type) =>
								return Err(DKGError::OfflineMisbehaviour {
									bad_actors: vec_usize_to_u16(err_type.bad_actors),
								}),
							gg20_sign::rounds::Error::Round3(err_type) =>
								return Err(DKGError::OfflineMisbehaviour {
									bad_actors: vec_usize_to_u16(err_type.bad_actors),
								}),
							gg20_sign::rounds::Error::Round5(err_type) =>
								return Err(DKGError::OfflineMisbehaviour {
									bad_actors: vec_usize_to_u16(err_type.bad_actors),
								}),
							gg20_sign::rounds::Error::Round6VerifyProof(err_type) =>
								return Err(DKGError::OfflineMisbehaviour {
									bad_actors: vec_usize_to_u16(err_type.bad_actors),
								}),
							_ =>
								return Err(DKGError::GenericError {
									reason: proceed_err.to_string(),
								}),
						},
						_ => return Err(DKGError::GenericError { reason: err.to_string() }),
					};
				},
			}
		}

		let (_, blame_vec) = offline_stage.round_blame();

		if self.try_finish_offline_stage(key.clone()) {
			Ok(true)
		} else {
			if at - *self.offline_started_at.get(&key).unwrap_or(&0u32.into()) >
				OFFLINE_TIMEOUT.into()
			{
				if !blame_vec.is_empty() {
					return Err(DKGError::OfflineTimeout { bad_actors: blame_vec })
				} else {
					// Should never happen
					warn!(target: "dkg", "🕸️  Offline timeout reached, but no missing parties found", );
				}
			}
			Ok(false)
		}
	}

	/// Try finish current Stage

	fn try_finish_offline_stage(&mut self, key: Vec<u8>) -> bool {
		if let Some(offline_stage) = self.offline_stage.get_mut(&key) {
			if offline_stage.is_finished() {
				info!(target: "dkg", "🕸️  OfflineStage is finished for {:?}, extracting output", &key);
				match offline_stage.pick_output() {
					Some(Ok(cos)) => {
						self.local_stages.insert(key.clone(), MiniStage::ManualReady);
						self.completed_offline_stage.insert(key.clone(), cos);
						info!(target: "dkg", "🕸️  CompletedOfflineStage is extracted");
					},
					Some(Err(e)) => info!("OfflineStage finished with error result {}", e),
					None => info!("OfflineStage finished with no result"),
				}
			}
		}

		if self.completed_offline_stage.contains_key(&key) {
			self.offline_stage.remove(&key);
			return true
		}
		false
	}

	/// Get outgoing messages for current Stage

	fn get_outgoing_messages_offline_stage(&mut self) -> Vec<DKGOfflineMessage> {
		let mut messages = vec![];
		trace!(target: "dkg", "🕸️  Getting outgoing offline messages");
		for (key, offline_stage) in self.offline_stage.iter_mut() {
			if !offline_stage.message_queue().is_empty() {
				trace!(target: "dkg", "🕸️  Outgoing messages for {:?}, queue len: {}", key, offline_stage.message_queue().len());

				let signer_set_id = self.signer_set_id;

				for m in offline_stage.message_queue().into_iter() {
					trace!(target: "dkg", "🕸️  MPC protocol message {:?}", *m);
					let serialized = serde_json::to_string(&m).unwrap();
					let msg = DKGOfflineMessage {
						key: key.clone(),
						signer_set_id,
						offline_msg: serialized.into_bytes(),
					};

					messages.push(msg);
				}

				offline_stage.message_queue().clear();
			}
		}

		messages
	}

	/// Handle incoming messages for current Stage

	fn handle_incoming_offline_stage(&mut self, data: DKGOfflineMessage) -> Result<(), DKGError> {
		if data.signer_set_id != self.signer_set_id {
			return Err(DKGError::GenericError { reason: "Signer set ids do not match".to_string() })
		}

		if let Some(offline_stage) = self.offline_stage.get_mut(&data.key) {
			trace!(target: "dkg", "🕸️  Handle incoming offline message");
			if data.offline_msg.is_empty() {
				warn!(target: "dkg", "🕸️  Got empty message");
				return Ok(())
			}
			let msg: Msg<OfflineProtocolMessage> = match serde_json::from_slice(&data.offline_msg) {
				Ok(msg) => msg,
				Err(err) => {
					error!(target: "dkg", "🕸️  Error deserializing msg: {:?}", err);
					return Err(DKGError::GenericError {
						reason: "Error deserializing offline msg".to_string(),
					})
				},
			};

			if Some(offline_stage.party_ind()) != msg.receiver &&
				(msg.receiver.is_some() || msg.sender == offline_stage.party_ind())
			{
				warn!(target: "dkg", "🕸️  Ignore messages sent by self");
				return Ok(())
			}
			trace!(
				target: "dkg", "🕸️  Party {} got message from={}, broadcast={}: {:?}",
				offline_stage.party_ind(),
				msg.sender,
				msg.receiver.is_none(),
				msg.body,
			);
			debug!(target: "dkg", "🕸️  State before incoming message processing: {:?}", offline_stage);
			match offline_stage.handle_incoming(msg.clone()) {
				Ok(()) => (),
				Err(err) if err.is_critical() => {
					error!(target: "dkg", "🕸️  Critical error encountered: {:?}", err);
					return Err(DKGError::CriticalError {
						reason: "Offline critical error encountered".to_string(),
					})
				},
				Err(err) => {
					error!(target: "dkg", "🕸️  Non-critical error encountered: {:?}", err);
				},
			}
			debug!(target: "dkg", "🕸️  State after incoming message processing: {:?}", offline_stage);
		}
		Ok(())
	}
}
