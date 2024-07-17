pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

use anchor_lang::prelude::*;

declare_id!("ErkZd8tVJs5UfcqHnJLz2iRxS37zXxp1CbQGGaKqEBDc");

#[program]
pub mod lending_borrowing {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    pub fn lend(ctx: Context<Lend>, amount: u64) -> Result<()> {
        instructions::lend::lend(ctx, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64, collateral: u64, interest_rate: u8) -> Result<()> {
        instructions::borrow::borrow(ctx, amount, collateral, interest_rate)
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        instructions::repay::repay(ctx, amount)
    }

    pub fn liquidate(ctx: Context<Liquidate>, borrower: Pubkey, min_collateral_ratio: u64, penalty: u64) -> Result<()> {
        instructions::liquidate::liquidate(ctx, borrower, min_collateral_ratio, penalty)
    }

    pub fn update_interest_rate(ctx: Context<UpdateInterestRate>, base_rate: u8, rate_multiplier: u8) -> Result<()> {
        instructions::update_interest_rate::update_interest_rate(ctx, base_rate, rate_multiplier)
    }

    pub fn propose(ctx: Context<Propose>) -> Result<()> {
        instructions::propose::propose(ctx)
    }

    pub fn vote(ctx: Context<Vote>, in_favor: bool) -> Result<()> {
        instructions::vote::vote(ctx, in_favor)
    }

    pub fn deposit_to_insurance_fund(ctx: Context<DepositToInsuranceFund>, amount: u64) -> Result<()> {
        instructions::deposit_to_insurance_fund::deposit_to_insurance_fund(ctx, amount)
    }
}
