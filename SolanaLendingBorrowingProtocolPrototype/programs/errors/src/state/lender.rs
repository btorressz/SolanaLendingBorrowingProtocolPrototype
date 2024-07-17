use anchor_lang::prelude::*;

#[account]
pub struct Lender {
    pub lender: Pubkey,
    pub amount_lent: u64,
}
