/* program specific errors */
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")] //thiserror is doing the fmt::Display implementation for us
    InvalidInstruction,

    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,

    /// Expected Amount Mismatch
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,

    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,
}

//implementing a generic trait - "From" trait. The reason we do this conversion is that the entrypoint returns a Result of either nothing or a ProgramError.
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32) //ProgramError enum provides "Custom variant" --> used for converting our programs EscrowError into a ProgramError
    }
}
