use anchor_lang::error_code;

#[error_code]
pub enum ErrorEarn {
    #[msg("Invalid fund")]
    InvalidFund,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Insufficient fund")]
    InsufficientFund,
}