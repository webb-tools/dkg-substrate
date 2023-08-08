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

//! # Bridge Registry Module
//!
//! A module for maintaining bridge metadata or views over connected
//! sets of anchors.
//!
//! ## Overview
//!
//! The Bridge Registry module provides functionality maintaing and storing
//! metadata about existing bridges.
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Terminology
//!
//! ### Goals
//!
//! ## Interface
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;

mod benchmarking;
pub mod types;

mod weights;
use weights::WeightInfo;

use types::*;

use sp_std::{convert::TryInto, prelude::*, vec};

use frame_support::pallet_prelude::{ensure, DispatchError};
use sp_runtime::traits::{AtLeast32Bit, One, Zero};
use webb_proposals::{evm::AnchorUpdateProposal, Proposal, ProposalKind, ResourceId};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::MaxEncodedLen;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config {
		/// The overarching RuntimeEvent type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The origin which may forcibly reset parameters or otherwise alter
		/// privileged attributes.
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Bridge index type
		type BridgeIndex: Encode
			+ Decode
			+ Parameter
			+ AtLeast32Bit
			+ Default
			+ Copy
			+ MaxEncodedLen;

		/// Maximum number of additional fields that may be stored in a bridge's metadata. Needed to
		/// bound the I/O required to access an identity, but can be pretty high.
		#[pallet::constant]
		type MaxAdditionalFields: Get<u32> + MaybeSerializeDeserialize;

		/// Maximum number of resources that may be stored in a bridge. This is not to be confused
		/// with the actual maximum supported by the bridge. Needed to bound the I/O
		/// required to access a metadata object, but can be pretty high.
		#[pallet::constant]
		type MaxResources: Get<u32> + MaybeSerializeDeserialize;

		/// Max length of a proposal
		#[pallet::constant]
		type MaxProposalLength: Get<u32>;

		/// Weight information for the extrinsics
		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub phantom: PhantomData<T>,
		pub bridges: Vec<BridgeMetadata<T::MaxResources, T::MaxAdditionalFields>>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			NextBridgeIndex::<T>::put(T::BridgeIndex::one());
			for bridge in &self.bridges {
				let idx: T::BridgeIndex = NextBridgeIndex::<T>::get();
				Bridges::<T>::insert(idx, bridge);
				NextBridgeIndex::<T>::put(idx + T::BridgeIndex::one());
				for rid in &bridge.resource_ids {
					ResourceToBridgeIndex::<T>::set(rid, Some(idx));
				}
			}
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn next_bridge_index)]
	/// Storage for next bridge index
	pub(super) type NextBridgeIndex<T: Config> = StorageValue<_, T::BridgeIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn bridges)]
	/// Storage for map of all bridges
	pub(super) type Bridges<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::BridgeIndex,
		BridgeMetadata<T::MaxResources, T::MaxAdditionalFields>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn resource_to_bridge_index)]
	/// Mapping of resource to bridge index
	pub(super) type ResourceToBridgeIndex<T: Config> =
		StorageMap<_, Blake2_256, ResourceId, T::BridgeIndex>;

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Parameters haven't been initialized
		ParametersNotInitialized,
		/// Error during verification
		VerifyError,
		/// Proposal is not signed and should not be processed
		ProposalNotSigned,
		/// Resources map to different bridge indices
		BridgeIndexError,
		/// Too many additional fields.
		TooManyFields,
		/// Bridge does not exist.
		BridgeNotFound,
		/// Too many resources.
		TooManyResources,
		/// Input out of bounds
		OutOfBounds,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set an account's identity information and reserve the appropriate deposit.
		///
		/// If the account already has identity information, the deposit is taken as part payment
		/// for the new deposit.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `info`: The identity information.
		///
		/// Emits `ResourceSet` if successful.
		#[pallet::weight(<T as Config>::WeightInfo::set_metadata())]
		#[pallet::call_index(0)]
		pub fn set_metadata(
			origin: OriginFor<T>,
			bridge_index: T::BridgeIndex,
			info: BridgeInfo<T::MaxAdditionalFields>,
		) -> DispatchResultWithPostInfo {
			T::ForceOrigin::ensure_origin(origin)?;
			let extra_fields = info.additional.len() as u32;
			ensure!(extra_fields <= T::MaxAdditionalFields::get(), Error::<T>::TooManyFields);

			let metadata = match <Bridges<T>>::get(bridge_index) {
				Some(mut id) => {
					id.info = info;
					id
				},
				None => BridgeMetadata { info, resource_ids: BoundedVec::default() },
			};

			<Bridges<T>>::insert(bridge_index, metadata);

			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::force_reset_indices(resource_ids.len() as u32))]
		#[pallet::call_index(1)]
		pub fn force_reset_indices(
			origin: OriginFor<T>,
			resource_ids: Vec<ResourceId>,
			bridge_index: T::BridgeIndex,
		) -> DispatchResultWithPostInfo {
			T::ForceOrigin::ensure_origin(origin)?;
			let max_resources = T::MaxResources::get() as usize;
			// Ensure that the resource_ids vector is within bounds
			if resource_ids.len() > max_resources {
				return Err(Error::<T>::TooManyResources.into())
			}
			for resource_id in resource_ids {
				ResourceToBridgeIndex::<T>::insert(resource_id, bridge_index);
			}

			Ok(().into())
		}
	}
}

/// A signed proposal handler implementation based on building bridge metadata.
///
/// This handler assumes that the bridge is being built incrementally as a single
/// connected component. If the bridge is built over a set of anchors and at any point
/// in the construction there are MORE than one connected component, this will throw
/// an error and the extrinsic will be rejected.
///
/// Note: There MUST only be a single connected component unless the end-user/developer wants
/// to utilize governance to fix the issue. This can be done using `force_reset_indices`.
use dkg_runtime_primitives::traits::OnSignedProposal;
impl<T: Config> OnSignedProposal<T::MaxProposalLength> for Pallet<T> {
	fn on_signed_proposal(proposal: Proposal<T::MaxProposalLength>) -> Result<(), DispatchError> {
		ensure!(proposal.is_signed(), Error::<T>::ProposalNotSigned);

		if proposal.kind() == ProposalKind::AnchorUpdate {
			// Decode the anchor update
			let data = proposal.data();
			let mut buf = [0u8; AnchorUpdateProposal::LENGTH];
			buf.clone_from_slice(data.as_slice());
			let anchor_update_proposal = AnchorUpdateProposal::from(buf);
			// Get the source and target resource IDs to check existence of
			let src_resource_id = anchor_update_proposal.src_resource_id();
			let dest_resource_id = anchor_update_proposal.header().resource_id();
			// Get the respective bridge indices
			let src_bridge_index =
				ResourceToBridgeIndex::<T>::get(src_resource_id).unwrap_or_default();
			let dest_bridge_index =
				ResourceToBridgeIndex::<T>::get(dest_resource_id).unwrap_or_default();
			// Ensure constraints on the bridge indices. If we are linking two anchors then:
			// 1. If we haven't assigned these resources, at least one of them must be zero.
			// 2. If we have assigned both resources, they must be the same.
			if src_bridge_index == T::BridgeIndex::zero() ||
				dest_bridge_index == T::BridgeIndex::zero()
			{
				// If both are zero, then we haven't assigned either resource.
				// We must create a new bridge index for these resources.
				if src_bridge_index == T::BridgeIndex::zero() &&
					dest_bridge_index == T::BridgeIndex::zero()
				{
					// Get the next bridge index
					let next_bridge_index = NextBridgeIndex::<T>::get();
					// Assign the bridge index to the source resource
					ResourceToBridgeIndex::<T>::insert(src_resource_id, next_bridge_index);
					// Assign the bridge index to the destination resource
					ResourceToBridgeIndex::<T>::insert(dest_resource_id, next_bridge_index);
					// Create the bridge record
					let bridge_metadata = BridgeMetadata {
						info: Default::default(),
						resource_ids: vec![src_resource_id, dest_resource_id]
							.try_into()
							.map_err(|_| Error::<T>::OutOfBounds)?,
					};
					Bridges::<T>::insert(next_bridge_index, bridge_metadata);
					// Increment the next bridge index
					NextBridgeIndex::<T>::mutate(|next_bridge_index| {
						*next_bridge_index += T::BridgeIndex::one();
					});
				} else {
					// We must connect the two resources to the same bridge.
					let (r_id, bridge_index) = if src_bridge_index == T::BridgeIndex::zero() {
						(src_resource_id, dest_bridge_index)
					} else {
						(dest_resource_id, src_bridge_index)
					};
					ResourceToBridgeIndex::<T>::insert(r_id, bridge_index);
					let mut metadata =
						Bridges::<T>::get(bridge_index).ok_or(Error::<T>::BridgeNotFound)?;
					metadata
						.resource_ids
						.try_push(r_id)
						.map_err(|_| Error::<T>::TooManyResources)?;
					Bridges::<T>::insert(bridge_index, metadata);
				}
			} else {
				ensure!(src_bridge_index == dest_bridge_index, Error::<T>::BridgeIndexError);
			}
		};

		Ok(())
	}
}
