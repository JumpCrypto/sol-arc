use anchor_lang::prelude::*;

#[error_code]
pub enum ComponentError {
    #[msg("Invalid Data Length!")]
    InvalidDataLengthError,
}