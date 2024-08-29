use anchor_lang::error_code;

#[error_code]
pub enum ErrorMath {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Divide by zero")]
    DivideByZero,
}