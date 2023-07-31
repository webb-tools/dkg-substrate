// This file is part of Webb.

// Copyright (C) 2021 Webb Technologies Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Verifier pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks_instance_pallet, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use webb_proposals::ResourceId;

benchmarks_instance_pallet! {
	set_metadata {
		let bridge_index = 0_u32;
		let metadata : BridgeInfo<T::MaxAdditionalFields> = Default::default();
	}: _(RawOrigin::Root, bridge_index.into(), metadata)
	verify {
		assert_eq!(
			Bridges::<T, I>::get::<T::BridgeIndex>(bridge_index.into()).unwrap(),
			BridgeMetadata {
				resource_ids: Default::default(),
				info: Default::default()
			}
		);
	}


	force_reset_indices {
		let mut resource_ids : Vec<ResourceId> = vec![];
	/// <HB SBP Review #3
	///
	/// This benchmark is not doing what it's supposed to do. As the bridge_index and resources_ids have always the same values
	/// (1_u32 and [0u8;32].into() respectively) the value is catched resulting in one write only (when it should be 1000 writes in this case)
	///
	/// If you check the results in the weight.rs file, it only shows one write when in this case is supposed to be 1000 writes.
	/// This might lead to failures in the block production where if the lenght of the vector is bigger than expected, that might
	/// overflow the block weight.
	///
	/// Try changing the push statement to:
	///
	/// resource_ids.push(i.encode().into())
	///
	/// and regenarate the weight.rs file.
	/// >
		for i in 0..1000 {
			resource_ids.push([0u8;32].into())
		}
		let bridge_index = 1_u32;
	}: _(RawOrigin::Root, resource_ids, bridge_index.into())
	verify {
		assert_eq!(
			ResourceToBridgeIndex::<T, I>::get::<ResourceId>([0u8; 32].into()).unwrap(),
			bridge_index.into()
		);
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
