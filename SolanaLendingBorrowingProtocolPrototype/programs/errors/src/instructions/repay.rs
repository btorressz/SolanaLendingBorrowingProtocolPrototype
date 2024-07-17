use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::borrower::Borrower;
use crate::utils::calculate_borrowed_value;

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,
    #[account(mut)]
    pub borrower_account: Account<'info, Borrower>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
    let borrower_account = &mut ctx.accounts.borrower_account;

    let loan_duration = (Clock::get()?.unix_timestamp - borrower_account.loan_start_time) as u64;
    let amount_owed = calculate_borrowed_value(borrower_account.amount_borrowed, borrower_account.interest_rate, loan_duration);

    require!(amount >= amount_owed, ErrorCode::InsufficientRepayment);

    // Transfer tokens from borrower to the protocol's token account
    let cpi_accounts = Transfer {
        from: ctx.accounts.token_account.to_account_info(),
        to: ctx.accounts.borrower_account.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Update borrower's state
    borrower_account.amount_borrowed = 0;
    borrower_account.collateral = 0;

    // Emit Repay Event
    emit!(RepayEvent {
        borrower: ctx.accounts.borrower.key(),
        amount,
    });

    Ok(())
}

#[event]
pub struct RepayEvent {
    pub borrower: Pubkey,
    pub amount: u64,
}
