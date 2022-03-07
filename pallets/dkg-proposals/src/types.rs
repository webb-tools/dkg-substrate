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

//! # DKG Proposals Module
//! The pallet provides functionality for voting on what proposals should be signed by the DKG.
//!
//! ## Overview
//! The dkg-proposal pallet provides functions for:
//! - Setting proposers threshold
//! - Registering and unregistering resources
//! - Registering and unregistering proposers
//! - Whitelisting chains
//!
//! It also provides util functionality for:
//! - Ensuring only admins can make the call
//! - Checking if the account making the call is a proposer
//! - Checking if a resource exists
//! - Checking if a chain is whitelisted
//! - Providing an accountId
//!
//! ### Terminology
//! - **Proposer**: A account that is trying to participate in the voting
//!
//! ### Goals
//!
//! The DKG proposal system is designed to make the following
//! possible:
//!
//! * Allowing voting on what is going to be signed by the DKG.
//!
//! ## Interface
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{prelude::*, vec};

pub const DKG_DEFAULT_PROPOSER_THRESHOLD: u32 = 1;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum ProposalStatus {
	Initiated,
	Approved,
	Rejected,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ProposalVotes<AccountId, BlockNumber> {
	pub votes_for: Vec<AccountId>,
	pub votes_against: Vec<AccountId>,
	pub status: ProposalStatus,
	pub expiry: BlockNumber,
}

impl<A: PartialEq, B: PartialOrd + Default> ProposalVotes<A, B> {
	/// Attempts to mark the proposal as approve or rejected.
	/// Returns true if the status changes from active.
	pub fn try_to_complete(&mut self, threshold: u32, total: u32) -> ProposalStatus {
		if self.votes_for.len() >= threshold as usize {
			self.status = ProposalStatus::Approved;
			ProposalStatus::Approved
		} else if total >= threshold && self.votes_against.len() as u32 + threshold > total {
			self.status = ProposalStatus::Rejected;
			ProposalStatus::Rejected
		} else {
			ProposalStatus::Initiated
		}
	}

	/// Returns true if the proposal has been rejected or approved, otherwise
	/// false.
	pub fn is_complete(&self) -> bool {
		self.status != ProposalStatus::Initiated
	}

	/// Returns true if `who` has voted for or against the proposal
	pub fn has_voted(&self, who: &A) -> bool {
		self.votes_for.contains(&who) || self.votes_against.contains(&who)
	}

	/// Return true if the expiry time has been reached
	pub fn is_expired(&self, now: B) -> bool {
		self.expiry <= now
	}
}

impl<AccountId, BlockNumber: Default> Default for ProposalVotes<AccountId, BlockNumber> {
	fn default() -> Self {
		Self {
			votes_for: vec![],
			votes_against: vec![],
			status: ProposalStatus::Initiated,
			expiry: BlockNumber::default(),
		}
	}
}
