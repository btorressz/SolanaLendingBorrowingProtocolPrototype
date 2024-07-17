use anchor_lang::prelude::*;
use crate::state::governance::Proposal;

#[derive(Accounts)]
pub struct Propose<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(init, payer = proposer, space = 8 + std::mem::size_of::<Proposal>())]
    pub proposal: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}

pub fn propose(ctx: Context<Propose>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    *proposal = Proposal::new(ctx.accounts.proposer.key());

    Ok(())
}
