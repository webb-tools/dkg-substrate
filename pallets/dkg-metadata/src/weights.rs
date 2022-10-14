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
//

//! Autogenerated weights for `pallet_dkg_metadata`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-10, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/dkg-standalone-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=10
// --log=warn
// --pallet=pallet-dkg-metadata
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/dkg-metadata/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn set_signature_threshold() -> Weight;
	fn set_keygen_threshold() -> Weight;
	fn set_refresh_delay(_n:u32,) -> Weight;
	fn manual_increment_nonce() -> Weight;
	fn manual_refresh() -> Weight;
	fn submit_public_key(n:u32,) -> Weight;
	fn submit_next_public_key(n:u32,) -> Weight;
	fn submit_misbehaviour_reports(n:u32,) -> Weight;
	fn submit_public_key_signature() -> Weight;
	fn unjail() -> Weight;
	fn force_unjail_signing() -> Weight;
	fn force_unjail_keygen() -> Weight;
}

/// Weight functions for `pallet_dkg_metadata`.
pub struct WebbWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for WebbWeight<T> {
	// Storage: DKG NextAuthorities (r:1 w:0)
	// Storage: DKG PendingSignatureThreshold (r:1 w:1)
	fn set_signature_threshold() -> Weight {
		Weight::from_ref_time(16_000_000)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKG NextAuthorities (r:1 w:0)
	// Storage: DKG PendingSignatureThreshold (r:1 w:0)
	// Storage: DKG PendingKeygenThreshold (r:1 w:1)
	fn set_keygen_threshold() -> Weight {
		Weight::from_ref_time(27_000_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKG RefreshDelay (r:0 w:1)
	fn set_refresh_delay(_n: u32, ) -> Weight {
		Weight::from_ref_time(2_110_000)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKG RefreshInProgress (r:1 w:0)
	// Storage: DKG RefreshNonce (r:1 w:1)
	fn manual_increment_nonce() -> Weight {
		Weight::from_ref_time(9_000_000)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKG RefreshInProgress (r:1 w:1)
	// Storage: DKG NextDKGPublicKey (r:1 w:0)
	// Storage: DKG RefreshNonce (r:1 w:1)
	// Storage: DKG ShouldManualRefresh (r:0 w:1)
	// Storage: DKGProposalHandler UnsignedProposalQueue (r:0 w:1)
	fn manual_refresh() -> Weight {
		Weight::from_ref_time(30_000_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: DKG DKGPublicKey (r:1 w:1)
	// Storage: DKG Authorities (r:1 w:0)
	// Storage: DKG SignatureThreshold (r:1 w:0)
	// Storage: DKG AuthoritySetId (r:1 w:0)
	// Storage: DKG AuthorityReputations (r:3 w:3)
	fn submit_public_key(n: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 22_593_000
			.saturating_add(Weight::from_ref_time((2_352_338_000_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}
	// Storage: DKG NextDKGPublicKey (r:1 w:1)
	// Storage: DKG NextAuthorities (r:1 w:0)
	// Storage: DKG NextSignatureThreshold (r:1 w:0)
	// Storage: DKG NextAuthoritySetId (r:1 w:0)
	fn submit_next_public_key(n: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 22_659_000
			.saturating_add(Weight::from_ref_time((2_358_378_000_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKG NextDKGPublicKey (r:1 w:1)
	// Storage: DKG NextPublicKeySignature (r:1 w:1)
	// Storage: DKG UsedSignatures (r:1 w:1)
	// Storage: DKG RefreshNonce (r:1 w:0)
	// Storage: DKG DKGPublicKey (r:1 w:1)
	// Storage: DKGProposalHandler UnsignedProposalQueue (r:1 w:0)
	// Storage: DKG ShouldManualRefresh (r:1 w:1)
	// Storage: DKG NextAuthorities (r:1 w:1)
	// Storage: DKG NextAuthoritiesAccounts (r:1 w:1)
	// Storage: DKGProposals AuthorityProposers (r:1 w:1)
	// Storage: DKGProposals ProposerCount (r:1 w:1)
	// Storage: DKG PendingSignatureThreshold (r:1 w:0)
	// Storage: DKG PendingKeygenThreshold (r:1 w:0)
	// Storage: DKG NextAuthoritySetId (r:1 w:1)
	// Storage: DKG NextBestAuthorities (r:1 w:1)
	// Storage: DKG JailedKeygenAuthorities (r:3 w:0)
	// Storage: DKG AuthorityReputations (r:3 w:0)
	// Storage: DKG DKGPublicKeySignature (r:1 w:1)
	// Storage: DKG AuthoritySetId (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	// Storage: DKG NextSignatureThreshold (r:0 w:1)
	// Storage: DKG Authorities (r:0 w:1)
	// Storage: DKG RefreshInProgress (r:0 w:1)
	// Storage: DKG KeygenThreshold (r:0 w:1)
	// Storage: DKG HistoricalRounds (r:0 w:1)
	// Storage: DKG PreviousPublicKey (r:0 w:1)
	// Storage: DKG SignatureThreshold (r:0 w:1)
	// Storage: DKG NextKeygenThreshold (r:0 w:1)
	// Storage: DKG BestAuthorities (r:0 w:1)
	// Storage: DKG CurrentAuthoritiesAccounts (r:0 w:1)
	// Storage: DKGProposals ExternalProposerAccounts (r:0 w:3)
	// Storage: DKGProposals ExternalAuthorityProposerAccounts (r:0 w:1)
	// Storage: DKGProposals Proposers (r:0 w:3)
	fn submit_public_key_signature() -> Weight {
		Weight::from_ref_time(383_000_000)
			.saturating_add(T::DbWeight::get().reads(24_u64))
			.saturating_add(T::DbWeight::get().writes(31_u64))
	}
	// Storage: DKG NextAuthorities (r:1 w:0)
	// Storage: DKG NextSignatureThreshold (r:1 w:0)
	// Storage: DKG AuthorityReputations (r:1 w:1)
	// Storage: DKG NextBestAuthorities (r:1 w:0)
	// Storage: DKG JailedKeygenAuthorities (r:3 w:1)
	// Storage: DKG NextKeygenThreshold (r:1 w:0)
	fn submit_misbehaviour_reports(n: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 23_711_000
			.saturating_add(Weight::from_ref_time((2_358_101_000_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DKG AccountToAuthority (r:1 w:0)
	// Storage: DKG JailedKeygenAuthorities (r:1 w:1)
	// Storage: DKG JailedSigningAuthorities (r:1 w:1)
	fn unjail() -> Weight {
		Weight::from_ref_time(22_000_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DKG JailedSigningAuthorities (r:0 w:1)
	fn force_unjail_signing() -> Weight {
		Weight::from_ref_time(2_000_000)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKG JailedKeygenAuthorities (r:0 w:1)
	fn force_unjail_keygen() -> Weight {
		Weight::from_ref_time(2_000_000)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

impl WeightInfo for () {

	fn set_signature_threshold() -> Weight { Weight::from_ref_time(0) }
	fn set_keygen_threshold() -> Weight { Weight::from_ref_time(0) }
	fn set_refresh_delay(_n: u32,) -> Weight { Weight::from_ref_time(0) }
	fn manual_increment_nonce() -> Weight { Weight::from_ref_time(0) }
	fn manual_refresh() -> Weight { Weight::from_ref_time(0) }
	fn submit_public_key(_n:u32,) -> Weight { Weight::from_ref_time(0) }
	fn submit_next_public_key(_n:u32,) -> Weight { Weight::from_ref_time(0) }
	fn submit_misbehaviour_reports(_n:u32,) -> Weight { Weight::from_ref_time(0) }
	fn submit_public_key_signature() -> Weight { Weight::from_ref_time(0) }
	fn unjail() -> Weight { Weight::from_ref_time(0) }
	fn force_unjail_signing() -> Weight { Weight::from_ref_time(0) }
	fn force_unjail_keygen() -> Weight { Weight::from_ref_time(0) }
}
