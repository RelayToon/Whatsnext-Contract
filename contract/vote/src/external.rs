use near_sdk::{ext_contract, AccountId};

#[ext_contract(ext_vote_controller)]
pub trait ExtVoteController{
    fn end_voting(&mut self, community_id: AccountId);
}