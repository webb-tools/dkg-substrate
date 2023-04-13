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

//! Autogenerated weights for `pallet_dkg_proposals`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/dkg-standalone-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet-dkg-proposals
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --json
// --output
// ./pallets/dkg-proposals/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn set_maintainer() -> Weight;
	fn force_set_maintainer() -> Weight;
	fn set_threshold(_c: u32, ) -> Weight;
	fn set_resource(_c: u32, ) -> Weight;
	fn remove_resource() -> Weight;
	fn whitelist_chain() -> Weight;
	fn add_proposer() -> Weight;
	fn remove_proposer() -> Weight;
	fn acknowledge_proposal(_c: u32,) -> Weight;
	fn reject_proposal(_c: u32,) -> Weight;
	fn eval_vote_state(_c: u32,) -> Weight;
}

	/// <HB SBP Review M2
	///
	/// The code is running under the release 0.9.39 however the weights are still using weights v1
	/// since it is considering only one value (ref time). The output of the CLI should configure the weights
	/// to use the `from_parts(ref time,proof size)` fn
	///
	/// HB SBP Review M2>

/// Weight functions for `pallet_dkg_proposals`.
pub struct WebbWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for WebbWeight<T> {
	// Storage: DKGProposals Maintainer (r:1 w:1)
	fn set_maintainer() -> Weight {
		Weight::from_ref_time(11_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals Maintainer (r:1 w:1)
	fn force_set_maintainer() -> Weight {
		Weight::from_ref_time(10_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals ProposerThreshold (r:0 w:1)
	fn set_threshold(_c: u32, ) -> Weight {
		Weight::from_ref_time(7_949_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals Resources (r:0 w:1)
	fn set_resource(_c: u32, ) -> Weight {
		Weight::from_ref_time(1_266_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals Resources (r:0 w:1)
	fn remove_resource() -> Weight {
		Weight::from_ref_time(1_000_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals ChainNonces (r:1 w:1)
	fn whitelist_chain() -> Weight {
		Weight::from_ref_time(11_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals Proposers (r:1 w:1)
	// Storage: DKGProposals ProposerCount (r:1 w:1)
	fn add_proposer() -> Weight {
		Weight::from_ref_time(12_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DKGProposals Proposers (r:1 w:1)
	// Storage: DKGProposals ProposerCount (r:1 w:1)
	fn remove_proposer() -> Weight {
		Weight::from_ref_time(13_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DKGProposals Proposers (r:1 w:0)
	// Storage: DKGProposals ChainNonces (r:1 w:0)
	// Storage: DKGProposals Resources (r:1 w:0)
	// Storage: DKGProposals Votes (r:1 w:1)
	// Storage: DKGProposals ProposerThreshold (r:1 w:0)
	// Storage: DKGProposals ProposerCount (r:1 w:0)
	fn acknowledge_proposal(_c: u32, ) -> Weight {
		Weight::from_ref_time(27_981_000_u64)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals Proposers (r:1 w:0)
	// Storage: DKGProposals ChainNonces (r:1 w:0)
	// Storage: DKGProposals Resources (r:1 w:0)
	// Storage: DKGProposals Votes (r:1 w:1)
	// Storage: DKGProposals ProposerThreshold (r:1 w:0)
	// Storage: DKGProposals ProposerCount (r:1 w:0)
	fn reject_proposal(_c: u32, ) -> Weight {
		Weight::from_ref_time(28_205_000_u64)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DKGProposals Votes (r:1 w:1)
	// Storage: DKGProposals ProposerThreshold (r:1 w:0)
	// Storage: DKGProposals ProposerCount (r:1 w:0)
	fn eval_vote_state(_c: u32, ) -> Weight {
		Weight::from_ref_time(10_859_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

}

impl WeightInfo for () {
	fn set_maintainer() -> Weight { Weight::from_ref_time(0) }
	fn force_set_maintainer() -> Weight { Weight::from_ref_time(0) }
	fn set_threshold(_c: u32, ) -> Weight { Weight::from_ref_time(0) }
	fn set_resource(_c: u32, ) -> Weight { Weight::from_ref_time(0) }
	fn remove_resource() -> Weight { Weight::from_ref_time(0) }
	fn whitelist_chain() -> Weight { Weight::from_ref_time(0) }
	fn add_proposer() -> Weight { Weight::from_ref_time(0) }
	fn remove_proposer() -> Weight { Weight::from_ref_time(0) }
	fn acknowledge_proposal(_c: u32,) -> Weight { Weight::from_ref_time(0) }
	fn reject_proposal(_c: u32,) -> Weight { Weight::from_ref_time(0) }
	fn eval_vote_state(_c: u32,) -> Weight { Weight::from_ref_time(0) }
}
