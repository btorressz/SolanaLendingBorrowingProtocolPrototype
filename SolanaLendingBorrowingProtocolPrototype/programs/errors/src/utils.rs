pub fn calculate_collateral_value(collateral: u64) -> u64 {
    collateral * 2 // Example: collateral value is twice the amount
}

pub fn calculate_borrowed_value(amount_borrowed: u64, interest_rate: u8, duration: u64) -> u64 {
    // Calculate interest based on the rate and duration
    let interest = (amount_borrowed as f64 * interest_rate as f64 * duration as f64 / 100.0) as u64;
    amount_borrowed + interest
}
