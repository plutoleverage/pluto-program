use anchor_lang::error_code;

#[error_code]
pub enum ErrorLeverage {
    #[msg["vault frozen"]]
    VaultFrozen,
    #[msg("invalid protocol fee")]
    InvalidProtocolFee,
    #[msg("min leverage must be greater than global min leverage and less than or equal to max leverage and greater than 1")]
    InvalidMinLeverage,
    #[msg("max leverage must be greater than or equal to min leverage and less than or equal to global max leverage")]
    InvalidMaxLeverage,
    #[msg("invalid max leverage limit, must be greater than min leverage limit")]
    InvalidMaxLeverageLessThanMinLeverage,
    #[msg("leverage step must be greater than 0")]
    InvalidLeverageStep,
    #[msg("invalid max leverage limit, must be greater than min leverage limit")]
    InvalidMaxLeverageLimitLessThanMinLeverage,
    #[msg("invalid max leverage limit")]
    InvalidMaxLeverageLimit,
    #[msg("invalid min leverage limit")]
    InvalidMinLeverageLimit,
    #[msg("invalid max deleverage limit, must be greater than min deleverage limit")]
    InvalidMaxDeleverageLessThanMinDeleverage,
    #[msg("invalid max deleverage limit")]
    InvalidMaxDeleverageLimit,
    #[msg("invalid min deleverage limit")]
    InvalidMinDeleverageLimit,
    #[msg("spread rate must be greater than 0")]
    InvalidSpreadRate,
    #[msg("saver threshold must be greater than 0")]
    InvalidSaverThreshold,
    #[msg("saver target must be greater leverage step")]
    InvalidSaverTargetLessThanLeverageStep,
    #[msg("saver target must be multiple of leverage step")]
    InvalidSaverTargetNotMultipleOfLeverageStep,
    #[msg("saver target must be greater than 0")]
    InvalidSaverTarget,
    #[msg("invalid amount")]
    InvalidAmount,
    #[msg("insufficient fund")]
    InsufficientFund,
    #[msg("insufficient borrowable amount")]
    InsufficientBorrowableAmount,
    #[msg("insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("invalid leverage")]
    InvalidLeverage,
    #[msg("profit taking rate must be greater than profit target rate")]
    ProfitTakingRateMustBeLessThanProfitTargetRate,
    #[msg("invalid profit taking rate")]
    InvalidProfitTakingRate,
    #[msg("invalid leverage min movement")]
    InvalidLeverageMinMovement,
    #[msg("current profit less than profit target")]
    CurrentProfitLessThanProfitTarget,
    #[msg("no position slot available")]
    NoPositionSlotAvailable,
    #[msg("no position found")]
    NoPositionFound,
    #[msg("invalid position number")]
    InvalidPositionNumber,
    #[msg("emergency eject are disabled")]
    EmergencyEjectDisabled,
    #[msg("safety mode are disabled")]
    SafetyModeDisabled,
    #[msg("profit taker are disabled")]
    ProfitTakerDisabled,

    #[msg("invalid indexer")]
    InvalidIndexer,
    #[msg("invalid keeper")]
    InvalidKeeper,

    #[msg("index factor is zero, holding until next update")]
    IndexFactorIsZero,

    #[msg("saver only works for reducing leverage")]
    SaverOnlyWorksForReducingLeverage,

    #[msg("no pending funded position found")]
    NoPendingFundedPositionFound,
    #[msg("no pending leveraged position found")]
    NoPendingLeveragedPositionFound,

    #[msg("slippage reached")]
    SlippageReached,

    #[msg("Missing Jupiter Swap")]
    MissingJupiterSwap,
    #[msg("Missing fund")]
    MissingFund,
    #[msg("Missing borrow")]
    MissingBorrow,
    #[msg("Missing take")]
    MissingTake,
    #[msg("Missing confiscate")]
    MissingConfiscate,
    #[msg("Cannot fund before confiscate")]
    CannotFundBeforeConfiscate,
    #[msg("Cannot borrow before confiscate")]
    CannotBorrowBeforeConfiscate,
    #[msg("Cannot take before borrow")]
    CannotTakeBeforeBorrow,
    #[msg("Cannot take before confiscate")]
    CannotTakeBeforeConfiscate,
    #[msg("Cannot confiscate before borrow")]
    CannotConfiscateBeforeBorrow,
    #[msg("Cannot confiscate before take")]
    CannotConfiscateBeforeTake,
    #[msg("Cannot pay liquidation fee before liquidate")]
    CannotPayLiquidationFeeBeforeLiquidate,

    #[msg("Missing release")]
    MissingRelease,
    #[msg("Missing store")]
    MissingStore,
    #[msg("Missing pay protocol")]
    MissingPayProtocol,
    #[msg("Missing repay borrow")]
    MissingRepayBorrow,
    #[msg("Missing closing")]
    MissingClosing,
    #[msg("Missing cleanup")]
    MissingCleanup,

    #[msg("Next instruction must be release")]
    NextInstructionMustBeRelease,
    #[msg("Next instruction must be repay borrow")]
    NextInstructionMustBeRepayBorrow,
    #[msg("Next instruction must be closing")]
    NextInstructionMustBeClosing,
    #[msg("Next instruction must be cleanup")]
    NextInstructionMustBeCleanup,

    #[msg("Missing keeper release")]
    MissingKeeperRelease,
    #[msg("Missing keeper repay borrow")]
    MissingKeeperRepayBorrow,
    #[msg("Missing keeper closing")]
    MissingKeeperClosing,

    #[msg("Next instruction must be keeper release")]
    NextInstructionMustBeKeeperRelease,
    #[msg("Next instruction must be keeper repay borrow")]
    NextInstructionMustBeKeeperRepayBorrow,
    #[msg("Next instruction must be keeper closing or keeper pay liquidation fee")]
    NextInstructionMustBeKeeperClosingOrKeeperPayLiquidationFee,
    #[msg("Next instruction must be keeper closing")]
    NextInstructionMustBeKeeperClosing,

    #[msg("Liquidation failed due to health factor")]
    UnmetHealthFactorThreshold
}