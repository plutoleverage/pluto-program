use crate::error::ErrorMath;
use anchor_lang::prelude::*;

pub const INDEX_DECIMALS: u8 = 12;
pub const INDEX_MULTIPLICATION: u64 = 10u64.pow(12);
pub const PERCENT_DECIMALS: u8 = 3;
pub const PERCENT_MULTIPLICATION: u64 = 10u64.pow(3);
pub const UNIT_DECIMALS: u8 = 6;
pub const UNIT_MULTIPLICATION: u64 = 10u64.pow(6);
pub const LAMPORTS_DECIMALS: u8 = 9;
pub const LAMPORTS_MULTIPLICATION: u64 = 10u64.pow(9);

pub const MAX_DECIMALS: u8 = 38;
pub const MAX_DECIMALS_64: u8 = 20;

pub fn mul( t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    let res_decimals = a_decimals + b_decimals;
    if t_decimals > res_decimals {
        return a.checked_mul(b).ok_or(ErrorMath::MathOverflow)?.checked_div(10u128.pow((t_decimals - res_decimals) as u32)).ok_or(Error::from(ErrorMath::DivideByZero));
    } else if t_decimals < res_decimals{
        return a.checked_mul(b).ok_or(ErrorMath::MathOverflow)?.checked_mul(10u128.pow((res_decimals - t_decimals) as u32)).ok_or(Error::from(ErrorMath::DivideByZero));
    } else {
        return a.checked_mul(b).ok_or(Error::from(ErrorMath::MathOverflow));
    }
}

pub fn div( t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    msg!("t_decimals: {}", t_decimals);
    msg!("a: {}", a);
    msg!("a_decimals: {}", a_decimals);
    msg!("b: {}", b);
    msg!("b_decimals: {}", b_decimals);

    let total_decimals = a_decimals + b_decimals;
    msg!("total_decimals: {}", total_decimals);
    let scaled_a_decs = 10u64.pow((t_decimals + b_decimals) as u32);
    let scaled_a = a.checked_mul(scaled_a_decs as u128).ok_or(ErrorMath::MathOverflow)?;
    msg!("scaled_a: {}", scaled_a);
    let raw_res = scaled_a.checked_div(b).ok_or(ErrorMath::MathOverflow)?;
    msg!("raw_res: {}", raw_res);

    return raw_res.checked_div(10u128.pow(a_decimals as u32)).ok_or(Error::from(ErrorMath::DivideByZero));
}