pub mod vote_ext;
pub use crate::vote_ext::*;

const CODE: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/vote.wasm");

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, Gas, Promise, PromiseError};
use near_sdk::{require, env, log, near_bindgen};
use near_sdk::collections::{LookupSet, LookupMap};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoteController{
    communities: LookupSet<AccountId>,
    is_voting: LookupMap<AccountId, AccountId>
}

// 1 NEAR
const INITIAL_BALANCE: Balance = 5_000_000_000_000_000_000_000_000;
const TGAS: u64 = 1_000_000_000;

impl Default for VoteController{
    fn default()-> Self{
        Self{
            communities: LookupSet::new(b'i'),
            is_voting: LookupMap::new(b'i'),
        }
    }
}

#[near_bindgen]
impl VoteController{
    pub fn is_voting(&self, community_id: AccountId) -> bool{
        let is_voting : Option<AccountId> = self.is_voting.get(&community_id);

        if Some(is_voting) != None {
            return true;
        } else {
            return false
        };
    }

    pub fn end_voting(&mut self, community_id: AccountId){
        require!(self.is_voting(community_id.clone()), "This community is not voting");
        require!(
            env::predecessor_account_id() == self.is_voting.get(&community_id).unwrap(),
            "Not Authorized"
        );

        self.is_voting.remove(&community_id);
    }

    pub fn is_community(&self, community_id: AccountId) -> bool{
        self.communities.contains(&community_id)
    }

    pub fn add_community(&mut self, community_id: AccountId) -> bool{
        self.communities.insert(&community_id)
    }

    pub fn new_vote(&self, prefix: AccountId, community_id: AccountId) -> Promise{
        require!(self.communities.contains(&community_id.clone()), "Not valid community account");
        require!( !self.is_voting(community_id.clone()), "This community is Voting" );

        log!("Creating a new vote for {}", prefix.to_string());
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );

        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec())
        .then(
            ext_vote::ext(subaccount_id.clone())
            .with_static_gas(Gas(5*TGAS))
            .new(community_id.clone(), env::current_account_id().clone())
            .then(
                Self::ext(env::current_account_id())
                .with_static_gas(Gas(5*TGAS))
                .new_vote_callback(community_id.clone())
            )
        )
    }

    #[private]
    pub fn new_vote_callback(&mut self, community_id: AccountId){
        self.is_voting.insert(&community_id, &env::predecessor_account_id());
    }
}
