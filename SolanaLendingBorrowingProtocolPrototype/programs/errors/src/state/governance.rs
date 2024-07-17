use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub proposer: Pubkey,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
}

impl Proposal {
    pub fn new(proposer: Pubkey) -> Self {
        Self {
            proposer,
            votes_for: 0,
            votes_against: 0,
            executed: false,
        }
    }
}
