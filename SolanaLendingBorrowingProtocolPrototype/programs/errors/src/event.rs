use anchor_lang::prelude::*;

#[event]
pub struct LendEvent {
    pub lender: Pubkey,
    pub amount: u64,
}

#[event]
pub struct BorrowEvent {
    pub borrower: Pubkey,
    pub amount: u64,
    pub collateral: u64,
    pub interest_rate: u8,
}

#[event]
pub struct RepayEvent {
    pub borrower: Pubkey,
    pub amount: u64,
}

#[event]
pub struct LiquidateEvent {
    pub liquidator: Pubkey,
    pub borrower: Pubkey,
    pub amount: u64,
}
