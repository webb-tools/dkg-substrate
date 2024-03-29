# DKG Metadata Module

 A pallet to manage metadata about the DKG and DKG proposal system.

 ## Overview

 The DKG Metadata pallet manages the following metadata about the DKG and DKG proposal system:
 - The active DKG public key
 - The active DKG authorities
 - The next DKG public key
 - The next DKG authorities
 - The DKG signature threshold (the `t` in `t-out-of-n` threshold signatures)
 - The key refresh process
 - The misbehavior reporting process

 The pallet tracks authority changes after each session and updates the metadata for the
 current and next authority sets respectively. The pallet also exposes a way for the root origin
 to update the signature threshold used in the DKG signing system.

 The pallet is responsible with initiating the key refresh process for rotating the DKG keys
 across authority set changes and executing the rotation when a new session starts.

 The pallet tracks reputations of DKG authorities by providing extrinsics and storage for
 submitting misbehaviour reports about authorities that misbehave.

 ### Terminology

 - **Authority**: A DKG authority.
 - **Authority Set**: A set of DKG authorities.
 - **Active DKG public key**: The public key of the DKG protocol that is currently active.
 - **Next DKG public key**: The public key of the DKG protocol that will be active after the next
   authority set change.
 - **DKG signature threshold**: The `t` in `t-out-of-n` threshold signatures used in the DKG
   signing system.
 - **Misbehaviour Report**: A report of a DKG authority misbehaving.
 - **Refresh delay**: The length of time within a session that the key refresh protocol will be
   initiated.

 ### Implementation

 The pallet is implemented to track and maintain a variety of different pieces of data. It is the
 primary pallet used to store the active and next DKG public keys that are generated by the
 active and next authorities of the underlying blockchain protocol. A runtime API is used to
 expose the active and next authorities to the external workers responsible with executing the
 DKG and signing protocol.

 The pallet exposes a variety of extrinsics for updating metadata:
 - `set_threshold`: Allows a root-origin to update the DKG signature threshold used in the
   threshold signing protocol.
 - `set_refresh_delay`: Allows a root-origin to update the delay before the key refresh process
   is initiated.
 - `submit_misbehaviour_reports`: Allows authorities to submit misbehaviour reports. Once the
   `threshold` number of reports is submitted, the offending authority will lose reputation.
 - `submit_public_key`: Allows submitting of the genesis public key by the initial authorities of
   the DKG protocol.
 - `submit_next_public_key`: Allows submitting of the next public key by the next authorities of
   the DKG protocol.

 The refresh process is initiated `T::RefreshDelay` into each session. A RefreshProposal is
 created and sent directly to the `pallet-dkg-proposal-handler` for inclusion into the unsigned
 proposal queue and soon after for signing. Once this proposal is signed, the signature is posted
 back on-chain to this pallet which rotates the DKG keys when the session ends.

 The misbehaviour reporting process relies on a honest-threshold assumption. When DKG authorities
 misbehave offchain, any detecting authority can submit a report of the offending authority. If a
 `threshold` number of reports is submitted, then the offending authority will lose reputation.
 This reputation map is utilized by each DKG authority to ensure every authority can generate a
 deterministic signing set for the threshold signing protocols. The signing set is taken to
 initially be the top `t` DKG authorities by reputation.

 ## Related Modules

 * [`System`](https://github.com/paritytech/substrate/tree/master/frame/system)
 * [`Support`](https://github.com/paritytech/substrate/tree/master/frame/support)
 * [`DKG Proposals`](https://github.com/webb-tools/dkg-substrate/blob/664aebd10e6c1dc9e787a0465fd36b60e5e82c0d/pallets/dkg-proposals)
 * [`DKG Proposal Handler`](https://github.com/webb-tools/dkg-substrate/blob/664aebd10e6c1dc9e787a0465fd36b60e5e82c0d/pallets/dkg-proposal-handler)
