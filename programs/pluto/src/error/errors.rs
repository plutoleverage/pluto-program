use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("Only the owner can call this function")]
    NotOwner,
    #[msg("Only the creator can call this function")]
    NotCreator,
    #[msg("Only the indexer can call this function")]
    NotIndexer,
    #[msg("invalid protocol")]
    InvalidProtocol,
    #[msg("Invalid config")]
    InvalidConfig,
    #[msg("Invalid borrowing config")]
    InvalidBorrowingConfig,
    #[msg("Invalid price oracle")]
    InvalidPriceOracle,
    #[msg("amount must be greater than 0")]
    InvalidAmountZero,
    #[msg("Incomplete process")]
    IncompleteProcess,
    #[msg("Incomplete leveraging process")]
    IncompleteLeveragingProcess,
    #[msg("Incomplete deleveraging process")]
    IncompleteDeleveragingProcess,
    #[msg("price oracle error")]
    PriceOracleError,
    #[msg("Failed to serialize account")]
    TryToSerializeAccount,
    #[msg("Simulated for error")]
    SimulatedError,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Invalid address")]
    InvalidAddress,
    #[msg("Invalid program")]
    InvalidProgram,
    #[msg("Unknown instruction")]
    UnknownInstruction,
    #[msg("Invalid jupiter program")]
    InvalidJupiterProgram,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}