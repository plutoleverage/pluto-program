use std::ops::{Div, Mul};
use anchor_lang::{InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::ErrorMath::{MathOverflow, MathOverflow1, MathOverflow2};
use crate::util::constant;
use crate::util::constant::TIME_ONE_DAY;
use crate::util::fraction::Fraction;

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[zero_copy(unsafe)]
#[repr(C)]
pub struct Rate {
    pub last_updated: i64,
    pub last_value: u32,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 4],
    pub last_ema_hour_updated: i64,
    pub ema_hourly: u32,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 4],
    pub last_ema_day_updated: i64,
    pub ema_3d: u32,
    pub ema_7d: u32,
    pub ema_14d: u32,
    pub ema_30d: u32,
    pub ema_90d: u32,
    pub ema_180d: u32,
    pub ema_365d: u32,
    #[derivative(Debug = "ignore")]
    pub align2: [u8; 4],
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 7],
}

impl Default for Rate {
    fn default() -> Self {
        Self {
            last_updated: 0,
            last_value: 0,
            align0: [0; 4],
            last_ema_hour_updated: 0,
            ema_hourly: 0,
            align1: [0; 4],
            last_ema_day_updated: 0,
            ema_3d: 0,
            ema_7d: 0,
            ema_14d: 0,
            ema_30d: 0,
            ema_90d: 0,
            ema_180d: 0,
            ema_365d: 0,
            align2: [0; 4],
            padding1: [0; 7],
        }
    }
}

impl Rate {
    pub fn update_rate(&mut self, value: u32, timestamp: i64) -> Result<()> {
        self.last_value = value;
        self.last_updated = timestamp;

        let time_diff = timestamp - self.last_updated;
        if time_diff >= constant::TIME_ONE_HOUR {
            self.last_ema_hour_updated = timestamp.div(constant::TIME_ONE_HOUR).mul(constant::TIME_ONE_HOUR); // REMOVE MINUTES
            self.ema_hourly = Fraction::from_num(value).checked_mul(Self::alpha(24)).ok_or(MathOverflow)?.checked_add(Fraction::from_num(self.ema_hourly).checked_mul(Self::counter_alpha(24)).ok_or(MathOverflow)?).ok_or(MathOverflow)?.to_num();
        }

        let time_diff = timestamp - self.last_ema_hour_updated;
        if time_diff >= TIME_ONE_DAY {
            if self.last_ema_day_updated == 0 {
                self.last_ema_day_updated = timestamp.div(TIME_ONE_DAY).mul(TIME_ONE_DAY); // REMOVE HOURS
                self.ema_3d = value;
                self.ema_7d = value;
                self.ema_14d = value;
                self.ema_30d = value;
                self.ema_90d = value;
                self.ema_180d = value;
                self.ema_365d = value;
            } else {
                self.last_ema_day_updated = timestamp.div(TIME_ONE_DAY).mul(TIME_ONE_DAY); // REMOVE HOURS
                self.ema_3d = Fraction::from_num(value).checked_mul(Self::alpha(3)).ok_or(MathOverflow1)?.checked_add(Fraction::from_num(self.ema_3d).checked_mul(Self::counter_alpha(3)).ok_or(MathOverflow)?).ok_or(MathOverflow2)?.to_num();
                self.ema_7d = Fraction::from_num(value).checked_mul(Self::alpha(7)).ok_or(MathOverflow)?.checked_add(Fraction::from_num(self.ema_7d).checked_mul(Self::counter_alpha(7)).ok_or(MathOverflow)?).ok_or(MathOverflow)?.to_num();
                self.ema_14d = Fraction::from_num(value).checked_mul(Self::alpha(14)).ok_or(MathOverflow)?.checked_add(Fraction::from_num(self.ema_14d).checked_mul(Self::counter_alpha(14)).ok_or(MathOverflow)?).ok_or(MathOverflow)?.to_num();
                self.ema_30d = Fraction::from_num(value).checked_mul(Self::alpha(30)).ok_or(MathOverflow)?.checked_add(Fraction::from_num(self.ema_30d).checked_mul(Self::counter_alpha(30)).ok_or(MathOverflow)?).ok_or(MathOverflow)?.to_num();
                self.ema_90d = Fraction::from_num(value).checked_mul(Self::alpha(90)).ok_or(MathOverflow)?.checked_add(Fraction::from_num(self.ema_90d).checked_mul(Self::counter_alpha(90)).ok_or(MathOverflow)?).ok_or(MathOverflow)?.to_num();
                self.ema_180d = Fraction::from_num(value).checked_mul(Self::alpha(180)).ok_or(MathOverflow)?.checked_add(Fraction::from_num(self.ema_180d).checked_mul(Self::counter_alpha(180)).ok_or(MathOverflow)?).ok_or(MathOverflow)?.to_num();
                self.ema_365d = Fraction::from_num(value).checked_mul(Self::alpha(365)).ok_or(MathOverflow)?.checked_add(Fraction::from_num(self.ema_365d).checked_mul(Self::counter_alpha(365)).ok_or(MathOverflow)?).ok_or(MathOverflow)?.to_num();
            }
        }

        Ok(())
    }

    fn alpha(period: u64) -> Fraction {
        Fraction::from_num(2) / (Fraction::from_num(period+1))
    }

    fn counter_alpha(period: u64) -> Fraction {
        Fraction::from_num(1) - (Fraction::from_num(2) / (Fraction::from_num(period+1)))
    }
}