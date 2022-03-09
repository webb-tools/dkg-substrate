use crate::*;
use codec::{Decode, Encode};
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, scale_info::TypeInfo)]
pub struct RoundMetadata {
	pub curr_round_pub_key: Vec<u8>,
	pub next_round_pub_key: Vec<u8>,
	pub refresh_signature: Vec<u8>,
}
/// different types of aggregated public key
pub enum AggregatedPublicKeyType {
	/// represents the aggregated public keys on chain at genesis
	AggregatedPublicKeysAtGenesis,
	/// represents the next aggregated public keys on chain
	AggregatedPublicKeys,
}
