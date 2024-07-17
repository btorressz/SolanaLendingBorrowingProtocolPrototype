use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::borrower::Borrower;
use crate::utils::{calculate_borrowed_value, calculate_collateral_value};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,
    #[account(mut)]
    pub borrower_account: Account<'info, Borrower>,
    #[account(mut)]
    pub collateral_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn liquidate(ctx: Context<Liquidate>, borrower: Pubkey, min_collateral_ratio: u64, penalty: u64) -> Result<()> {
    let borrower_account = &mut ctx.accounts.borrower_account;

    let loan_duration = (Clock::get()?.unix_timestamp - borrower_account.loan_start_time) as u64;
    let borrowed_value = calculate_borrowed_value(borrower_account.amount_borrowed, borrower_account.interest_rate, loan_duration);
    let collateral_value = calculate_collateral_value(borrower_account.collateral);

    // Check if the borrower's collateral is insufficient
    require!(
        collateral_value < (borrowed_value * min_collateral_ratio / 100),
        ErrorCode::InsufficientCollateral
    );

    // Transfer collateral to liquidator with penalty
    let penalty_amount = (borrower_account.collateral * penalty / 100);
    let cpi_accounts = Transfer {
        from: ctx.accounts.collateral_account.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, borrower_account.collateral - penalty_amount)?;

    // Transfer penalty amount to liquidator
    let cpi_accounts = Transfer {
        from: ctx.accounts.collateral_account.to_account_info(),
        to: ctx.accounts.liquidator.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, penalty_amount)?;

    // Update borrower's state
    borrower_account.collateral = 0;
    borrower_account.amount_borrowed = 0;

    // Emit Liquidate Event
    emit!(LiquidateEvent {
        liquidator: ctx.accounts.liquidator.key(),
        borrower,
        collateral: borrower_account.collateral,
        penalty: penalty_amount,
    });

    Ok(())
}

#[event]
pub struct LiquidateEvent {
    pub liquidator: Pubkey,
    pub borrower: Pubkey,
    pub collateral: u64,
    pub penalty: u64,
}
