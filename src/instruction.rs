use borsh::{
    BorshDeserialize, 
    BorshSerialize
};
use solana_program::program_error::ProgramError;

use crate::{
    error::CNFTBurnerError, 
    state::BurnCNFTData,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum CNFTBurnerInstruction {
    BurnCNFT {
        burn_cnft_data: BurnCNFTData,
    },
}

impl CNFTBurnerInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(CNFTBurnerError::InvalidInstruction)?;
        Ok(match tag {
            0 => Self::BurnCNFT {
                burn_cnft_data: BurnCNFTData::try_from_slice(rest).unwrap()
            },
            _ => return Err(CNFTBurnerError::InvalidInstruction.into()),
        })
    }
}
