use anchor_lang::prelude::*;
use crate::state::insurance_fund::InsuranceFund;

#[derive(Accounts)]
pub struct DepositToInsuranceFund<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub insurance_fund: Account<'info, InsuranceFund>,
}

pub fn deposit_to_insurance_fund(ctx: Context<DepositToInsuranceFund>, amount: u64) -> Result<()> {
    let insurance_fund = &mut ctx.accounts.insurance_fund;
    insurance_fund.deposit(amount);

    Ok(())
}
