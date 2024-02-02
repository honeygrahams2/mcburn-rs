use borsh::{
    BorshDeserialize, 
    BorshSerialize
};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct BurnCNFTData {  // 32 + 32 + 32 + 32 + 8 + 1
    pub asset_id: Pubkey,
    pub root: Pubkey,
    pub data_hash: Pubkey,
    pub creator_hash: Pubkey,
    pub nonce: u64,
    pub proof_length: u8,
}