use anchor_lang::error_code;

#[error_code]
pub enum ErrorMath {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Math overflow 1")]
    MathOverflow1,
    #[msg("Math overflow 2")]
    MathOverflow2,
    #[msg("Math overflow 3")]
    MathOverflow3,
    #[msg("Divide by zero")]
    DivideByZero,
}