
//! Autogenerated weights for pallet_dkg_proposal_handler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-26, STEPS: `20`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/dkg-standalone-node
// benchmark
// pallet
// --chain=dev
// --steps=20
// --repeat=1
// --log=warn
// --pallet=pallet-dkg-proposal-handler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./pallets/dkg-proposal-handler/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_dkg_proposal_handler.
pub trait WeightInfo {
	fn submit_signed_proposals(n: u32, ) -> Weight;
	fn force_submit_unsigned_proposal() -> Weight;
}

/// Weights for pallet_dkg_proposal_handler using the Substrate node and recommended hardware.
pub struct WebbWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for WebbWeight<T> {
	/// Storage: DKG DKGPublicKey (r:1 w:0)
	/// Proof: DKG DKGPublicKey (max_values: Some(1), max_size: Some(522), added: 1017, mode: MaxEncodedLen)
	/// Storage: DKGProposalHandler UnsignedProposalQueue (r:100 w:100)
	/// Proof: DKGProposalHandler UnsignedProposalQueue (max_values: None, max_size: Some(20052), added: 22527, mode: MaxEncodedLen)
	/// Storage: DKGProposalHandler SignedProposals (r:0 w:100)
	/// Proof: DKGProposalHandler SignedProposals (max_values: None, max_size: Some(20048), added: 22523, mode: MaxEncodedLen)
	/// Storage: DKGProposalHandler NextUnsignedAt (r:0 w:1)
	/// Proof: DKGProposalHandler NextUnsignedAt (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn submit_signed_proposals(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `563 + n * (57 ±0)`
		//  Estimated: `1017 + n * (22527 ±0)`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(38_155_265, 1017)
			// Standard Error: 875_381
			.saturating_add(Weight::from_parts(90_756_705, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 22527).saturating_mul(n.into()))
	}
	/// Storage: DKGProposalHandler UnsignedProposalQueue (r:0 w:1)
	/// Proof: DKGProposalHandler UnsignedProposalQueue (max_values: None, max_size: Some(20052), added: 22527, mode: MaxEncodedLen)
	fn force_submit_unsigned_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DKG DKGPublicKey (r:1 w:0)
	/// Proof: DKG DKGPublicKey (max_values: Some(1), max_size: Some(522), added: 1017, mode: MaxEncodedLen)
	/// Storage: DKGProposalHandler UnsignedProposalQueue (r:100 w:100)
	/// Proof: DKGProposalHandler UnsignedProposalQueue (max_values: None, max_size: Some(20052), added: 22527, mode: MaxEncodedLen)
	/// Storage: DKGProposalHandler SignedProposals (r:0 w:100)
	/// Proof: DKGProposalHandler SignedProposals (max_values: None, max_size: Some(20048), added: 22523, mode: MaxEncodedLen)
	/// Storage: DKGProposalHandler NextUnsignedAt (r:0 w:1)
	/// Proof: DKGProposalHandler NextUnsignedAt (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn submit_signed_proposals(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `563 + n * (57 ±0)`
		//  Estimated: `1017 + n * (22527 ±0)`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(38_155_265, 1017)
			// Standard Error: 875_381
			.saturating_add(Weight::from_parts(90_756_705, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 22527).saturating_mul(n.into()))
	}
	/// Storage: DKGProposalHandler UnsignedProposalQueue (r:0 w:1)
	/// Proof: DKGProposalHandler UnsignedProposalQueue (max_values: None, max_size: Some(20052), added: 22527, mode: MaxEncodedLen)
	fn force_submit_unsigned_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}