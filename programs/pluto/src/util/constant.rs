use anchor_lang::prelude::Pubkey;
use anchor_lang::pubkey;

pub const INDEX_DECIMALS: u8 = 12;
pub const INDEX_ONE: u128 = 10u128.pow(12);
pub const PERCENT_DECIMALS: u8 = 3;
pub const PERCENT_ONE: u32 = 10u32.pow(3);
pub const PERCENT_MAX: u32 = 10u32.pow(5);
pub const UNIT_DECIMALS: u8 = 8;
pub const UNIT_ONE: u64 = 10u64.pow(8);
pub const LAMPORTS_DECIMALS: u8 = 9;
pub const LAMPORTS_ONE: u64 = 10u64.pow(9);

pub const FLOOR_CAP_RATIO: u32 = 80 * PERCENT_ONE; // 80%
pub const PROTOCOL_CAP_RATIO: u32 = 50 * PERCENT_ONE; // 50%

pub const MAX_DECIMALS: u8 = 38;
pub const MAX_DECIMALS_64: u8 = 20;

pub const MIN_LEVERAGE: u32 = 101; // 1.01
pub const MAX_LEVERAGE: u32 = 500 * 10u32.pow(2); // 500x
pub const LEVERAGE_STEP: u32 = 1 * 10u32.pow(2); // 500x

pub const MIN_DEPOSIT_LIMIT: u64 = 1;
pub const MAX_DEPOSIT_LIMIT: u64 = 1_000_000;
pub const MIN_WITHDRAW_LIMIT: u64 = 0;
pub const MAX_WITHDRAW_LIMIT: u64 = 10_000_000;
pub const MIN_BORROW_LIMIT: u64 = 0;
pub const MAX_BORROW_LIMIT: u64 = 100_000;

pub const TIME_ONE_MINUTE: i64 = 60;
pub const TIME_ONE_HOUR: i64 = 60 * TIME_ONE_MINUTE;
pub const TIME_ONE_DAY: i64 = 24 * TIME_ONE_HOUR;
pub const TIME_ONE_WEEK: i64 = 7 * TIME_ONE_DAY;
pub const TIME_ONE_MONTH: i64 = 30 * TIME_ONE_DAY;
pub const TIME_ONE_YEAR: i64 = 365 * TIME_ONE_DAY;

pub const LEVERAGE_MAX_SAFETY: u32 = 5000; // 5.00
pub const LEVERAGE_ONE: u32 = 1000; // 1.00

pub const MAX_OBLIGATION_POSITIONS: u8 = 3;

pub const MAX_ORACLE_AGE: u64 = 180;

pub const USDC_PRICE_FEEDS: &[u8; 64] = b"eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";

pub const WSOL_TOKEN_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const JUPITER_SWAP_PROGRAM_ID: Pubkey = pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

pub const MERCURY_PROGRAM_ID: Pubkey = pubkey!("E4HWdh2qyNYdjuQqvkXUyj73Z2FAnyq5w5PRDfDxzn59");
pub const MERCURY_AMM_ADDRESS: Pubkey = pubkey!("89AUkCRrbkrnDnUcdMHukfdRPeQDfJ1Tj6V8oZpWCbt5");
pub const MERCURY_POOL_ADDRESS: Pubkey = pubkey!("24r3X9TnsyzYfZDNycoSd2kUmJQeFW1KuZmiQxam67Zx");
pub const MERCURY_POOL_AUTHORITY_ADDRESS: Pubkey = pubkey!("76W2GRHvyA8HQJSWwvbywugdcSpSmqobNarHVYGWgxGg");

// Liquidation can execute only if HF is less than this value
pub const LIQUIDATION_HF_THRESHOLD: u8 = 1;