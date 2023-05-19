mod external;
use crate::external::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, Balance, Gas, Promise, PromiseError, PanicOnDefault, near_bindgen};
use near_sdk::{log, require, env};
use near_sdk::ext_contract;
use near_sdk::collections::UnorderedMap;
use near_contract_standards::fungible_token::core::ext_ft_core;

const TGAS: u64 = 1_000_000_000_000;
const SECONDS: u64 = 1_000_000_000;
const VOTE_PERIOD: u64 = 24 * 3600 * SECONDS;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Vote{
    votes_proposal: UnorderedMap<u64, Balance>,
    votes_available: UnorderedMap<AccountId, Balance>,
    total_votes: Balance,
    proposals: UnorderedMap<u64, String>,
    result: Option<u64>,
    end_date: u64,
    vote_controller_account_id: AccountId,
    community_account_id: AccountId,
    is_voting: bool,
}

pub enum Status{
    Init, Voting, End
}

#[ext_contract(ext_vote)]
pub trait ExtVote{
    fn ping(&mut self);
    fn vote(&mut self, is_vote: bool);
    fn get_total_voted_stake(&self) -> (U128, U128);
    fn get_votes(&self) -> UnorderedMap<AccountId, U128>;
}

#[near_bindgen]
impl Vote {
    #[init]
    pub fn new(community_account_id: AccountId, vote_controller_account_id: AccountId) -> Self{
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            votes_proposal: UnorderedMap::new(b"d"),
            votes_available: UnorderedMap::new(b"d"),
            total_votes: 0,
            proposals: UnorderedMap::new(b"d"),
            result: None,
            end_date: env::block_timestamp() + VOTE_PERIOD,
            vote_controller_account_id,
            community_account_id,
            is_voting: true,
        }
    }

    #[private]
    pub fn get_ft_balance_of(&self, account_id : AccountId)-> Promise {
        let promise = ext_ft_core::ext(self.community_account_id.clone())
            .with_static_gas(Gas(5*TGAS))
            .ft_balance_of(account_id);

        return promise;
    }

    //votes end
    #[private]
    pub fn end(&mut self)-> Promise {
        self.is_voting = false;
        // NFT Connecting
        ext_vote_controller::ext(self.vote_controller_account_id.clone())
        .with_static_gas(Gas(5*TGAS))
        .end_voting(self.community_account_id.clone())
    }

    pub fn ping(&mut self){
        let current_time : u64 = env::block_timestamp();
        if current_time > self.end_date {
            self.end();
        }
    }

    pub fn register(&mut self)->Promise{
        let signer : AccountId = env::signer_account_id();
        let is_register: Option<Balance> = self.votes_available.get(&signer);
        require!(is_register == None, "Already Registered");

        ext_ft_core::ext(self.community_account_id.clone())
        .ft_balance_of(env::predecessor_account_id().clone())
        .then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .register_callback()
        )
    }

    #[private]
    pub fn register_callback(&mut self, #[callback_result] balance_result: Result<Balance, PromiseError>){
        if balance_result.is_err() {
            return
        } else {
            let signer = env::signer_account_id();
            self.votes_available.insert(&signer, &balance_result.unwrap());
        }
    }

    pub fn vote(&mut self, proposal: U64, amount: Balance){
        self.ping();
        if self.is_voting == true {
            log!("Voting is already ended");
            return
        }

        let signer_id : AccountId = env::predecessor_account_id();
        let is_account_registered: Option<Balance> = self.votes_available.get(&signer_id);
        require!( Some(is_account_registered) != None , "Not registered account");

        let num_proposal : u64 = self.proposals.len();
        let proposal = u64::from(proposal.0);
        require!(proposal <= num_proposal, "Not Validate Proposal ID");

        // Votes to Proposal
        let current_proposal_votes = self.votes_proposal.get(&proposal).unwrap_or(0);
        let added_proposal_votes = amount + current_proposal_votes;
        self.votes_proposal.insert(&proposal, &(added_proposal_votes));
        
        // Adding Total Votes with signer account
        let votes_available_with_signer: u128 = is_account_registered.unwrap();
        require!(votes_available_with_signer > amount, "Not enough votes");
        self.votes_available.insert(&signer_id, &(votes_available_with_signer-amount));

        // Adding total votes for this contract.
        self.total_votes += amount;

        // Changing current max candidate
        if self.result == None {
            self.result = Some(proposal);
        } else {
            let current_max_votes: Balance = self.votes_proposal.get(&self.result.unwrap_or(0)).unwrap_or(0);
            if added_proposal_votes > current_max_votes {
                self.result = Some(proposal);
            }
        }
    }

    #[private]
    pub fn vote_callback(&mut self, #[callback_result] call_result: Result<U128, PromiseError>) -> bool{
        // Bound to restrict voter by the balances owned
        return false;
    }

    pub fn add_proposal(&mut self, proposal_url:String) -> u64{
        let length: u64 = self.proposals.len();
        self.proposals.insert(&(length+1), &proposal_url);
        
        self.proposals.len()
    }

    pub fn get_all_proposals(&self)-> Vec<(u64, String)>{
        self.proposals.to_vec()
    }

    pub fn get_proposals_length(&self) -> U64{
        U64(self.proposals.len())
    }

    pub fn get_result(&self) -> Option<u64>{
        self.result.clone()
    }

    pub fn get_total_votes(&self) -> U128{
        U128( self.total_votes )
    }

    pub fn get_votes_proposal(&self, proposal : U64) -> U128{
        U128( self.votes_proposal.get(&u64::from(proposal.0)).unwrap_or(0).try_into().unwrap() )
    }

    pub fn get_votes_available(&self, account_id : AccountId) -> U128{
        U128( self.votes_available.get(&account_id).unwrap_or(0) )
    }

    pub fn get_is_voting(&self) -> bool{
        self.is_voting
    }

    pub fn get_community_account_id(&self) -> AccountId{
        self.community_account_id.clone()
    }
}