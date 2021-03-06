#![warn(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(feature = "std")]
extern crate serde;

extern crate parity_codec as codec;

extern crate sr_std as rstd;
extern crate sr_primitives as runtime_primitives;
extern crate substrate_primitives as primitives;

use runtime_primitives::generic;
use runtime_primitives::{OpaqueExtrinsic, traits::BlakeTwo256};

/// An index to a block.
pub type BlockNumber = u64;

/// Alias to Ed25519 pubkey that identifies an account on the chain. This will almost
/// certainly continue to be the same as the substrate's `AuthorityId`.
pub type AccountId = ::primitives::H256;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// The Ed25519 pub key of an session that belongs to an authority of the chain. This is
/// exactly equivalent to what the substrate calls an "authority".
pub type SessionKey = primitives::Ed25519AuthorityId;

/// Index of a transaction in the chain.
pub type Index = u64;

/// A hash of some data used by the chain.
pub type Hash = primitives::H256;

/// Alias to 512-bit hash when used in the context of a signature on the chain.
pub type Signature = runtime_primitives::Ed25519Signature;

/// A timestamp: seconds since the unix epoch.
pub type Timestamp = u64;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, generic::DigestItem<Hash, SessionKey>>;
/// Block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// Opaque, encoded, unchecked extrinsic.
pub type UncheckedExtrinsic = OpaqueExtrinsic;
