use anchor_lang::prelude::*;

#[account]
pub struct InterestRate {
    pub base_rate: u8,
    pub rate_multiplier: u8,
}

impl InterestRate {
    pub fn calculate_rate(&self, utilization_rate: u8) -> u8 {
        self.base_rate + self.rate_multiplier * utilization_rate / 100
    }
}
