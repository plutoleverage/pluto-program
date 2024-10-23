
pub fn position_size(collateral: u64, leverage: u16) -> u64 {
    collateral * leverage as u64
}

pub fn leverage_ratio(collateral: u64, borrowed: u64) -> u64 {
    (borrowed as u128 / collateral as u128 * 100) as u64
}