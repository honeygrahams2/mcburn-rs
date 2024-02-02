use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum CNFTBurnerError {
    #[error("CERROR: Invalid Instruction")]
    InvalidInstruction,
    #[error("CERROR: Invalid Instruction Data")]
    InvalidInstructionData,
    #[error("CERROR: Invalid Tree Authority")]
    InvalidTreeAuthority,
    #[error("CERROR: Invalid Asset ID")]
    InvalidAssetID,
    #[error("CERROR: Invalid SPL Token Program ID")]
    InvalidSPLTokenID,
    #[error("CERROR: Invalid SPL Account Compression Program ID")]
    InvalidSPLAccountCompressionProgramID,
    #[error("CERROR: Invalid SPL Noop Program ID")]
    InvalidSPLNoopProgramID,
    #[error("CERROR: Invalid System Program ID")]
    InvalidSystemProgramID,
    #[error("CERROR: Invalid MPL Bubblegum Program ID")]
    InvalidBubblegumProgramID,
}

impl From<CNFTBurnerError> for ProgramError {
    fn from(e: CNFTBurnerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}