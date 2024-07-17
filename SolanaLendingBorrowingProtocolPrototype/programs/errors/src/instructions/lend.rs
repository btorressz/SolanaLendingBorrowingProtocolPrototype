use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::lender::Lender;

#[derive(Accounts)]
pub struct Lend<'info> {
    #[account(mut)]
    pub lender: Signer<'info>,
    #[account(mut)]
    pub lender_account: Account<'info, Lender>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn lend(ctx: Context<Lend>, amount: u64) -> Result<()> {
    let lender_account = &mut ctx.accounts.lender_account;

    // Transfer tokens from lender to the protocol's token account
    let cpi_accounts = Transfer {
        from: ctx.accounts.token_account.to_account_info(),
        to: ctx.accounts.lender_account.to_account_info(),
        authority: ctx.accounts.lender.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Update lender's state
    lender_account.amount_lent += amount;

    // Emit Lend Event
    emit!(LendEvent {
        lender: ctx.accounts.lender.key(),
        amount,
    });

    Ok(())
}

#[event]
pub struct LendEvent {
    pub lender: Pubkey,
    pub amount: u64,
}
