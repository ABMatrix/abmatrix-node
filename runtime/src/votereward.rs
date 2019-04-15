//extern crate sr_primitives as primitives;
extern crate sr_io as runtime_io;
use rstd::prelude::Vec;
use {balances, system::{self, ensure_signed}};
use srml_support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure};
use runtime_primitives::traits::*;


/// The amount of exposure (to slashing) than an individual nominator has.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct NominatorInfo<AccountId, Balance> {
    /// The stash account of the nominator in question.
    who: AccountId,
    /// Amount of funds exposed.
    value: Balance,
    /// Amount of interest
    interest: Balance,
}


/// A snapshot of the stake backing a single validator in the system.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct VotePool <Accountid , Balance>{
    /// The total balance backing this validator.  这个validator的票池里所有人押金总和，用来算比例
    pub total: Balance,
    /// The validator's own stash that is exposed.   这个validator自身的的押金
    pub own: Balance,
    pub interest: Balance,
    /// The portions of nominators stashes that are exposed.    这个validator旗下nominators每个人的押金
    pub others: Vec<NominatorInfo<Accountid, Balance>>,
}

pub trait Trait: balances::Trait + session::Trait + staking::Trait{
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
        Txisok(Hash,AccountId),
        // 交易 = vec<id，签名>
        //TranscationVerified(Hash,Vec<(AccountId,Hash)>),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Vote {

        /// 记录每个交易的签名的数量
        /// Record the number of signatures per transaction got
        NumberOfSignedContract get(num_of_signed): map T::Hash => u64;

        /// 需要这些数量的签名，才发送这个交易通过的事件
        /// These amount of signatures are needed to send the event that the transaction verified.
        MinNumOfSignature get(min_signature)  : u64 = 1;

       ///  validator ---  List of nominators (id,money)
       pub VoteInfo get(vote_pool): map T::AccountId => VotePool<T::AccountId,T::Balance>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;

        /// 当验证者
        pub fn validate(origin,money: T::Balance){
            // 原本接口
            let who = ensure_signed(origin)?;
			ensure!(<staking::Module<T>>::nominating(&who).is_none(), "Cannot stake if already nominating.");
			let mut intentions = <staking::Module<T>>::intentions();
			// can't be in the list twice.
			ensure!(intentions.iter().find(|&t| t == &who).is_none(), "Cannot stake if already staked.");
    		//let bondage = <staking::Module<T>>::bondage();
    		//bondage.insert(&who, T::BlockNumber::max_value());
			//intentions.push(who);
			//<Intentions<T>>::put(intentions);
            // 新接口
        }

        fn nominate(origin, target: <T::Lookup as StaticLookup>::Source){

        }

        fn unvalidate(origin){

        }

        fn unnominate(origin){

        }
    }
}

impl<T: Trait> Module<T> {
    fn _verify(_tx : T::Hash) -> Result{
        //TODO:verify signature or others
        Ok(())
    }


}