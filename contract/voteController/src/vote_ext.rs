use near_sdk::{ext_contract, AccountId};

#[ext_contract(ext_vote)]
pub trait ExtVote{
    fn new(community_account_id: AccountId, vote_controller_account_id: AccountId);
}
