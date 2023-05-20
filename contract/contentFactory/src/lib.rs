// const CODE: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/cotent.wasm");

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, Gas, Promise, PromiseError};
use near_sdk::{require, env, log, near_bindgen};
use near_sdk::collections::{LookupSet, LookupMap, Vector};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContentFactory{
    contents: Vector<AccountId>,
}

// 1 NEAR
const INITIAL_BALANCE: Balance = 5_000_000_000_000_000_000_000_000;
const TGAS: u64 = 1_000_000_000;

impl Default for ContentFactory{
    fn default()-> Self{
        Self{
            contents: Vector::new(b"v".to_vec())
        }
    }
}

#[near_bindgen]
impl ContentFactory{
    pub fn new_vote(&self, prefix: AccountId, community_id: AccountId) -> Promise{
        require!(self.contents.contains(&community_id.clone()), "Not valid community account");
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
