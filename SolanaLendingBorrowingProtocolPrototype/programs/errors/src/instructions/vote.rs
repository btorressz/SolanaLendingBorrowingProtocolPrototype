use anchor_lang::prelude::*;
use crate::state::governance::Proposal;

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

pub fn vote(ctx: Context<Vote>, in_favor: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    if in_favor {
        proposal.votes_for += 1;
    } else {
        proposal.votes_against += 1;
    }

    Ok(())
}
