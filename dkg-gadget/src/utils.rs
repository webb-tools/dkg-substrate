use crate::{
	worker::{DKGWorker, ENGINE_ID},
	Client,
};
use codec::Codec;
use dkg_primitives::{
	crypto::AuthorityId, rounds::MultiPartyECDSARounds, AuthoritySet, ConsensusLog, DKGApi,
	MmrRootHash,
};
use dkg_runtime_primitives::crypto::Public;
use sc_client_api::Backend;
use sc_keystore::LocalKeystore;
use sp_api::{BlockT as Block, HeaderT};
use sp_arithmetic::traits::AtLeast32BitUnsigned;
use sp_core::sr25519;
use sp_runtime::{generic::OpaqueDigestItemId, traits::Header};
use std::{collections::HashMap, sync::Arc};

pub fn find_index<B: Eq>(queue: &[B], value: &B) -> Option<usize> {
	for (i, v) in queue.iter().enumerate() {
		if value == v {
			return Some(i)
		}
	}
	None
}

pub fn validate_threshold(n: u16, t: u16) -> u16 {
	let max_thresh = n - 1;
	if t >= 1 && t <= max_thresh {
		return t
	}

	return max_thresh
}

pub fn set_up_rounds<N: AtLeast32BitUnsigned + Copy>(
	authority_set: &AuthoritySet<AuthorityId>,
	public: &AuthorityId,
	sr25519_public: &sr25519::Public,
	thresh: u16,
	local_key_path: Option<std::path::PathBuf>,
	created_at: N,
	local_keystore: Option<Arc<LocalKeystore>>,
	reputations: &HashMap<AuthorityId, i64>,
) -> MultiPartyECDSARounds<N> {
	let party_inx = find_index::<AuthorityId>(&authority_set.authorities[..], public).unwrap() + 1;
	// Compute the reputations of only the currently selected authorities for these rounds
	let mut authority_set_reputations = HashMap::new();
	authority_set.authorities.iter().for_each(|id| {
		authority_set_reputations.insert(id.clone(), *reputations.get(id).unwrap_or(&0i64));
	});
	let n = authority_set.authorities.len();
	// Generate the rounds object
	let rounds = MultiPartyECDSARounds::builder()
		.round_id(authority_set.id.clone())
		.party_index(u16::try_from(party_inx).unwrap())
		.threshold(thresh)
		.parties(u16::try_from(n).unwrap())
		.local_key_path(local_key_path)
		.reputations(authority_set_reputations)
		.authorities(authority_set.authorities.clone())
		.build();

	rounds
}

/// Extract the MMR root hash from a digest in the given header, if it exists.
pub fn find_mmr_root_digest<B, Id>(header: &B::Header) -> Option<MmrRootHash>
where
	B: Block,
	Id: Codec,
{
	header.digest().logs().iter().find_map(|log| {
		match log.try_to::<ConsensusLog<Id>>(OpaqueDigestItemId::Consensus(&ENGINE_ID)) {
			Some(ConsensusLog::MmrRoot(root)) => Some(root),
			_ => None,
		}
	})
}

/// Scan the `header` digest log for a DKG validator set change. Return either the new
/// validator set or `None` in case no validator set change has been signaled.
pub fn find_authorities_change<B>(
	header: &B::Header,
) -> Option<(AuthoritySet<AuthorityId>, AuthoritySet<AuthorityId>)>
where
	B: Block,
{
	let id = OpaqueDigestItemId::Consensus(&ENGINE_ID);

	header.digest().convert_first(|l| l.try_to(id).and_then(match_consensus_log))
}

fn match_consensus_log(
	log: ConsensusLog<AuthorityId>,
) -> Option<(AuthoritySet<AuthorityId>, AuthoritySet<AuthorityId>)> {
	match log {
		ConsensusLog::AuthoritiesChange {
			next_authorities: validator_set,
			next_queued_authorities,
		} => Some((validator_set, next_queued_authorities)),
		_ => None,
	}
}

pub(crate) fn is_next_authorities_or_rounds_empty<B, C, BE>(
	mut dkg_worker: &mut DKGWorker<B, C, BE>,
	mut next_authorities: &AuthoritySet<Public>,
) -> bool
where
	B: Block,
	BE: Backend<B>,
	C: Client<B, BE>,
	C::Api: DKGApi<B, AuthorityId, <<B as Block>::Header as Header>::Number>,
{
	if next_authorities.authorities.is_empty() {
		return true
	}

	if dkg_worker.rounds.is_some() {
		if dkg_worker.rounds.as_ref().unwrap().get_id() == next_authorities.id {
			return true
		}
	}

	false
}

pub(crate) fn is_queued_authorities_or_rounds_empty<B, C, BE>(
	mut dkg_worker: &mut DKGWorker<B, C, BE>,
	mut queued_authorities: &AuthoritySet<Public>,
) -> bool
where
	B: Block,
	BE: Backend<B>,
	C: Client<B, BE>,
	C::Api: DKGApi<B, AuthorityId, <<B as Block>::Header as Header>::Number>,
{
	if queued_authorities.authorities.is_empty() {
		return true
	}

	if dkg_worker.next_rounds.is_some() {
		if dkg_worker.next_rounds.as_ref().unwrap().get_id() == queued_authorities.id {
			return true
		}
	}

	false
}
