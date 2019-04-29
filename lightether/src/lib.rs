#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

extern crate sr_io;
extern crate sr_std as rstd;
#[macro_use]
extern crate substrate_client as client;
#[macro_use]
extern crate srml_support as support;
extern crate sr_primitives as runtime_primitives;
#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;
extern crate substrate_primitives as primitives;
extern crate parity_codec;
#[macro_use]
extern crate parity_codec_derive;
#[macro_use]
extern crate sr_version as version;
extern crate srml_fees as fees;
extern crate srml_sudo as sudo;
extern crate srml_aura as aura;
extern crate srml_system as system;
extern crate srml_session as session;
extern crate srml_staking as staking;
extern crate srml_grandpa as grandpa;
extern crate srml_indices as indices;
extern crate srml_balances as balances;
extern crate srml_executive as executive;
extern crate srml_consensus as consensus;
extern crate srml_timestamp as timestamp;
extern crate srml_finality_tracker as finality_tracker;
extern crate substrate_consensus_aura_primitives as consensus_aura;


use support::{decl_storage, decl_module, StorageValue, StorageMap,
    dispatch::Result, ensure, decl_event, traits::Currency};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash, Zero};
use parity_codec::{Encode, Decode};
use rstd::cmp;
use rstd::prelude::Vec;

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Default, PartialEq, Clone, Encode, Decode)]
pub struct BestHeader{

}

#[derive(Default, PartialEq, Clone, Encode, Decode)]
pub struct BlockHeader {

}

#[derive(Default, PartialEq, Clone, Encode, Decode)]
pub struct H256 {

}

fn deserializate(header: Vec<u8>) -> BlockHeader {
    BlockHeader{}
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
        fn deposit_event<T>() = default;

        fn push_header(origin, header: Vec<u8>) -> Result {
            // runtime_io::print("[anchor eth] push eth header");
            let from = ensure_signed(origin)?;

            // deserializate header.
            let block_header = deserializate(header);
            // verify header.

            // storage.
            Ok(())
        }

        fn push_transaction(origin, tx: Vec<u8>) -> Result {
            // deserializate transaction

            // verify transaction

            // make a event
            Ok(())
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as LightEther {
        pub BestIndex get(best_index): BestHeader;

        // all valid blockheader (include orphan blockheader)
        pub BlockHeaderFor get(block_header_for): map H256 => Option<(BlockHeader, T::AccountId, T::BlockNumber)>;

        // only main chain could has this number
        pub NumberForHash get(num_for_hash): map H256 => Option<u32>;
        pub HashsForNumber get(hashs_for_num): map u32 => Vec<H256>;

        pub AnchorAddress get(anchor_address) : H256;
    }
}


decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as balances::Trait>::Balance
    {
        Free(Balance, AccountId),
    }
);


// ethtransaction --> internalTransaction
// btctransaction --> internalTransaction
// citatransaction --> internalTransaction
// 存储内部交易
struct InternalTransaction {
    tx_hash: H256,
    value: u64,
    from: H256,
    to: H256,
}

struct TireNode {
    hash: H256,
}

pub struct RelayTransaction {
    inter: InternalTransaction,
    proves: Vec<TireNode>,
    header_hash: H256,
}

impl<T: Trait> Module<T> {
    pub fn verify_header(header: &BlockHeader) -> bool {
        // let parent = best_index
        // verify parent hash
        // if parent.hash == header.parent_hash {
        //     return false;
        // }

        // verify timestamp
        // if parent.timestamp > header.timestamp {
        //     return false;
        // }

        // verify difficulty

        true
    }

    pub fn verify_transaction(tx: &RelayTransaction) -> bool {
        true
    }
}
