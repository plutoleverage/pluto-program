use anchor_lang::error_code;

#[error_code]
pub enum ErrorEarn {
    #[msg["vault frozen"]]
    VaultFrozen,
    #[msg("Invalid LTV, must be greater than 0")]
    InvalidLTV,
    #[msg("Invalid max deposit limit, must be greater than 0")]
    InvalidMaxDepositLimit,
    #[msg("Invalid max deposit limit, must be greater than min deposit limit")]
    InvalidMaxDepositLimitLessThanMinDepositLimit,
    #[msg("Invalid max withdraw limit")]
    InvalidMaxWithdrawLimit,
    #[msg("Invalid max withdraw limit, must be greater than min withdraw limit")]
    InvalidMaxWithdrawLimitLessThanMinWithdrawLimit,
    #[msg("Invalid max borrow limit")]
    InvalidMaxBorrowLimit,
    #[msg("Invalid max borrow limit, must be greater than min borrow limit")]
    InvalidMaxBorrowLimitLessThanMinBorrowLimit,
    #[msg("Invalid floor cap rate, must be greater than 0")]
    InvalidFloorCapRate,
    #[msg("Invalid fund")]
    InvalidFund,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Insufficient fund")]
    InsufficientFund,
    #[msg("Insufficient liquidity in pool")]
    InsufficientLiquidityInPool,
    #[msg("No pending unit")]
    NoPendingUnit,
    #[msg("Deposit min limit not met")]
    DepositMinLimitNotMet,
    #[msg("Deposit max limit exceeded")]
    DepositMaxLimitExceeded,
    #[msg("Withdraw min limit not met")]
    WithdrawMinLimitNotMet,
    #[msg("Withdraw max limit exceeded")]
    WithdrawMaxLimitExceeded,
    #[msg("Borrow min limit not met")]
    BorrowMinLimitNotMet,
    #[msg("Borrow max limit exceeded")]
    BorrowMaxLimitExceeded,
    #[msg("Output too small")]
    OutputTooSmall,

    #[msg("Missing pay protocol")]
    MissingPayProtocol,

    #[msg("Next instruction must be a pay protocol")]
    NextInstructionMustBePayProtocol,

    #[msg("index factor is zero, holding until next update")]
    IndexFactorIsZero,
}