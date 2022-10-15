// This file is part of Webb.

// Copyright (C) 2022 Webb Technologies Inc.
// SPDX-License-Identifier: Apache-2.0

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

//! Autogenerated weights for pallet_bridge_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-01, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Stanlys-MacBook-Air.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/webb-standalone-node
// benchmark
// pallet
// --chain=dev
// --steps=20
// --repeat=10
// --log=warn
// --pallet=pallet-bridge-registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/bridge-registry/src/weights.rs
// --template=./.maintain/webb-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bridge_registry.
pub trait WeightInfo {
	fn set_metadata() -> Weight;
	fn force_reset_indices() -> Weight;
}

/// Weights for pallet_bridge_registry using the Substrate node and recommended hardware.
pub struct WebbWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for WebbWeight<T> {
	// Storage: BridgeRegistry Bridges (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: BridgeRegistry ResourceToBridgeIndex (r:0 w:1)
	fn force_reset_indices() -> Weight {
		Weight::from_ref_time(862_000_000)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: BridgeRegistry Bridges (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_ref_time(6_000_000)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: BridgeRegistry ResourceToBridgeIndex (r:0 w:1)
	fn force_reset_indices() -> Weight {
		Weight::from_ref_time(862_000_000)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
