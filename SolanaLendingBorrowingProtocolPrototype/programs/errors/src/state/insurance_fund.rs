use anchor_lang::prelude::*;

#[account]
pub struct InsuranceFund {
    pub balance: u64,
}

impl InsuranceFund {
    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        if self.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        self.balance -= amount;
        Ok(())
    }
}
