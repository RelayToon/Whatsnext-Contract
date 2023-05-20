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
    is_vote: LookupMap<AccountId, AccountId>
}

// 1 NEAR
const INITIAL_BALANCE: Balance = 5_000_000_000_000_000_000_000_000;
const TGAS: u64 = 1_000_000_000_000;

impl Default for VoteController{
    fn default()-> Self{
        Self{
            communities: LookupSet::new(b"i"),
            is_vote: LookupMap::new(b"m"),
        }
    }
}

#[near_bindgen]
impl VoteController{
    pub fn is_voting(&self, community_id: AccountId) -> bool{
        let is_voting= self.is_vote.get(&community_id);
        if is_voting.is_some() {
            return true;
        } else {
            return false
        };
    }

    pub fn get_vote_account_id(&self, community_id: AccountId) -> AccountId{
        self.is_vote.get(&community_id).unwrap()
    }

    pub fn end_voting(&mut self, community_id: AccountId){
        require!(self.is_voting(community_id.clone()), "This community is not voting");
        require!(
            env::predecessor_account_id() == self.is_vote.get(&community_id).unwrap(),
            "Not Authorized"
        );

        self.is_vote.remove(&community_id);
    }

    pub fn is_community(&self, community_id: AccountId) -> bool{
        self.communities.contains(&community_id)
    }

    pub fn add_community(&mut self, community_id: AccountId) -> bool{
        self.communities.insert(&community_id)
    }

    pub fn new_vote(&self, prefix: String, community_id: AccountId) -> Promise{
        require!(self.communities.contains(&community_id.clone()), "Not valid community account");
        require!( !self.is_voting(community_id.clone()), "This community is Voting" );

        log!("Creating a new vote for {}", prefix.to_string());
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id().clone())
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
        ).then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .new_vote_callback(community_id.clone(), subaccount_id.clone())
        )
    }

    #[private]
    pub fn new_vote_callback(&mut self, community_id: AccountId, subaccount_id: AccountId){
        self.is_vote.insert(&community_id, &subaccount_id);
    }
}
