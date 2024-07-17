use anchor_lang::prelude::*;
use crate::state::interest_rate::InterestRate;

#[derive(Accounts)]
pub struct UpdateInterestRate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub interest_rate: Account<'info, InterestRate>,
}

pub fn update_interest_rate(ctx: Context<UpdateInterestRate>, base_rate: u8, rate_multiplier: u8) -> Result<()> {
    let interest_rate = &mut ctx.accounts.interest_rate;
    interest_rate.base_rate = base_rate;
    interest_rate.rate_multiplier = rate_multiplier;

    Ok(())
}
