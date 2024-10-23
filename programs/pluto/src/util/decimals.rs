use crate::error::ErrorMath;
use anchor_lang::prelude::*;
use fixed::traits::Fixed;
use crate::error::ErrorMath::{DivideByZero, MathOverflow, MathOverflow1, MathOverflow2};
use crate::util::constant::INDEX_ONE;
use crate::util::fraction::{Fraction};

pub fn mul(t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    let res_decimals = a_decimals + b_decimals;
    if t_decimals > res_decimals {
        return a.checked_mul(b).ok_or(MathOverflow)?.checked_div(10u128.pow((t_decimals - res_decimals) as u32)).ok_or(Error::from(DivideByZero));
    } else if t_decimals < res_decimals{
        return a.checked_mul(b).ok_or(MathOverflow)?.checked_div(10u128.pow((res_decimals - t_decimals) as u32)).ok_or(Error::from(DivideByZero));
    } else {
        return a.checked_mul(b).ok_or(Error::from(MathOverflow));
    }
}

pub fn mul_ceil(t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    let x = Fraction::from_num(a);
    let y = Fraction::from_num(b);

    let mut res = x.checked_mul(y.checked_div(Fraction::from_num(10u128.pow(b_decimals as u32))).ok_or(Error::from(MathOverflow))?).ok_or(Error::from(MathOverflow1))?;
    if (t_decimals as i8 - a_decimals as i8) > 0 {
        res = res.checked_mul(Fraction::from_num(10u128.pow((t_decimals - a_decimals) as u32))).ok_or(Error::from(MathOverflow2))?;
    } else if (t_decimals as i8 - a_decimals as i8) < 0 {
        res = res.checked_div(Fraction::from_num(10u128.pow((a_decimals - t_decimals) as u32))).ok_or(Error::from(DivideByZero))?;
    }

    Ok(res.to_num::<u128>())
}

pub fn mul_floor(t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    let x = Fraction::from_num(a);
    let y = Fraction::from_num(b);

    let mut res = x.checked_mul(y.checked_div(Fraction::from_num(10u128.pow(b_decimals as u32))).ok_or(Error::from(MathOverflow))?).ok_or(Error::from(MathOverflow1))?;
    if (t_decimals as i8 - a_decimals as i8) > 0 {
        res = res.checked_mul(Fraction::from_num(10u128.pow((t_decimals - a_decimals) as u32))).ok_or(Error::from(MathOverflow2))?;
    } else if (t_decimals as i8 - a_decimals as i8) < 0 {
        res = res.checked_div(Fraction::from_num(10u128.pow((a_decimals - t_decimals) as u32))).ok_or(Error::from(DivideByZero))?;
    }

    Ok(res.to_num::<u128>())
}

pub fn div( t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    let scaled_a_decs = 10u64.pow((t_decimals + b_decimals) as u32);
    let scaled_a = a.checked_mul(scaled_a_decs as u128).ok_or(MathOverflow)?;
    let raw_res = scaled_a.checked_div(b).ok_or(MathOverflow)?;

    return raw_res.checked_div(10u128.pow(a_decimals as u32)).ok_or(Error::from(DivideByZero));
}

pub fn div_ceil( t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    let x = Fraction::from_num(a).checked_div(Fraction::from_num(10u128.pow(a_decimals as u32))).ok_or(Error::from(DivideByZero))?;
    let y = Fraction::from_num(b).checked_div(Fraction::from_num(10u128.pow(b_decimals as u32))).ok_or(Error::from(DivideByZero))?;

    let res = x.checked_div(y).ok_or(Error::from(MathOverflow))?.checked_mul(Fraction::from_num(10u128.pow(t_decimals as u32))).ok_or(Error::from(MathOverflow))?.to_num::<u128>();

    Ok(res)
}

pub fn div_floor( t_decimals: u8, a: u128, a_decimals: u8, b: u128, b_decimals: u8) -> Result<u128> {
    let x = Fraction::from_num(a).checked_div(Fraction::from_num(10u128.pow(a_decimals as u32))).ok_or(Error::from(DivideByZero))?;
    let y = Fraction::from_num(b).checked_div(Fraction::from_num(10u128.pow(b_decimals as u32))).ok_or(Error::from(DivideByZero))?;

    let res = x.checked_div(y).ok_or(Error::from(MathOverflow))?.checked_mul(Fraction::from_num(10u128.pow(t_decimals as u32))).ok_or(Error::from(MathOverflow))?.to_num::<u128>();

    Ok(res)
}

pub fn pow(a: u128, b: u32) -> Result<u128> {
    let mut y = INDEX_ONE;
    for _ in 0..b {
        y = a.checked_mul(y).unwrap().checked_div(INDEX_ONE).unwrap()
    }
    Ok(y)
}