use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::borrower::Borrower

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,
    #[account(mut)]
    pub borrower_account: Account<'info, Borrower>,
    #[account(mut)]
    pub collateral_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn borrow(ctx: Context<Borrow>, amount: u64, collateral: u64, interest_rate: u8) -> Result<()> {
    let borrower_account = &mut ctx.accounts.borrower_account;

    // Transfer collateral from borrower to the protocol's collateral account
    let cpi_accounts = Transfer {
        from: ctx.accounts.collateral_account.to_account_info(),
        to: ctx.accounts.borrower_account.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, collateral)?;

    // Transfer tokens from protocol to the borrower
    let cpi_accounts = Transfer {
        from: ctx.accounts.borrower_account.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Update borrower's state
    borrower_account.amount_borrowed += amount;
    borrower_account.collateral += collateral;
    borrower_account.interest_rate = interest_rate;
    borrower_account.loan_start_time = Clock::get()?.unix_timestamp;

    // Emit Borrow Event
    emit!(BorrowEvent {
        borrower: ctx.accounts.borrower.key(),
        amount,
        collateral,
        interest_rate,
    });

    Ok(())
}

#[event]
pub struct BorrowEvent {
    pub borrower: Pubkey,
    pub amount: u64,
    pub collateral: u64,
    pub interest_rate: u8,
}

