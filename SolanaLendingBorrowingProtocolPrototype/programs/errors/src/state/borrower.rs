use anchor_lang::prelude::*;

#[account]
pub struct Borrower {
    pub borrower: Pubkey,
    pub amount_borrowed: u64,
    pub collateral: u64,
    pub interest_rate: u8,
}
