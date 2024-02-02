use solana_program::{
    account_info::{
        AccountInfo,
        next_account_info, 
    },
    entrypoint::ProgramResult,
    instruction::AccountMeta, 
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke, 
};
use std::str::FromStr;

use crate::{
    error::CNFTBurnerError, 
    instruction::CNFTBurnerInstruction,
    state::BurnCNFTData,
};

const BURN_DICRIMINATOR: &'static [u8;8] = &[116, 110, 29, 56, 107, 219, 42, 93];

pub fn assert_true(cond: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !cond {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

pub struct Processor {}
impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instr: CNFTBurnerInstruction = CNFTBurnerInstruction::unpack(instruction_data)?;
        match instr {
            CNFTBurnerInstruction::BurnCNFT {
                burn_cnft_data
            } => {
                msg!("Burning cNFT");
                Self::process_burn_cnft(
                    accounts,
                    burn_cnft_data,
                )
            },
        }?;

        Ok(())
    }

    fn process_burn_cnft(
        accounts: &[AccountInfo],
        burn_cnft_data: BurnCNFTData,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let signer = next_account_info(accounts_iter)?; // 0
        let leaf_delegate = next_account_info(accounts_iter)?; // 1
        let merkle_tree = next_account_info(accounts_iter)?; // 2
        let tree_authority_pda = next_account_info(accounts_iter)?; // 3
        let spl_account_compression_program_id = next_account_info(accounts_iter)?; // 4
        let log_wrapper_program_id = next_account_info(accounts_iter)?; // 5
        let system_program_id = next_account_info(accounts_iter)?; // 6
        let mpl_bubblegum_program_id = next_account_info(accounts_iter)?; // 7

        if !signer.is_signer {
            msg!("CERROR: Missing required signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let (tree_authority, _tree_authority_bump) = Pubkey::find_program_address(
            &[merkle_tree.key.as_ref()], 
            &mpl_bubblegum::ID,
        );
        assert_true(
            *tree_authority_pda.key == tree_authority,
            ProgramError::from(CNFTBurnerError::InvalidTreeAuthority),
            "CERROR: Invalid tree authority",
        )?;

        let (asset_id, _asset_id_bump) = Pubkey::find_program_address(
            &[
                b"asset", 
                merkle_tree.key.as_ref(), 
                &burn_cnft_data.nonce.to_le_bytes(),
            ], 
            &mpl_bubblegum::ID,
        );
        assert_true(
            asset_id == burn_cnft_data.asset_id,
            ProgramError::from(CNFTBurnerError::InvalidAssetID),
            "CERROR: Invalid asset id",
        )?;        
        
        assert_true(
            *spl_account_compression_program_id.key == spl_account_compression::id(),
            ProgramError::from(CNFTBurnerError::InvalidSPLAccountCompressionProgramID),
            "CERROR: Invalid SPL Account Compression Program ID",
        )?;
        
        assert_true(
            *log_wrapper_program_id.key == Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap(),
            ProgramError::from(CNFTBurnerError::InvalidSPLNoopProgramID),
            "CERROR: Invalid SPL Noop Program ID",
        )?;
        
        assert_true(
            *system_program_id.key == solana_program::system_program::id(),
            ProgramError::from(CNFTBurnerError::InvalidSystemProgramID),
            "CERROR: Invalid System Program ID",
        )?;

        assert_true(
            *mpl_bubblegum_program_id.key == mpl_bubblegum::ID,
            ProgramError::from(CNFTBurnerError::InvalidBubblegumProgramID),
            "CERROR: Invalid Bubblegum Program ID",
        )?;

        msg!("Burning: {}", burn_cnft_data.asset_id.to_string());
        let mut accounts:  Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new(*tree_authority_pda.key, false),  // treeAuthority
            AccountMeta::new(*signer.key, true),  // leafOwner
            AccountMeta::new(*leaf_delegate.key, false),  // leafDelegate
            AccountMeta::new(*merkle_tree.key, false),  // merkleTree
            AccountMeta::new_readonly(*log_wrapper_program_id.key, false),  // logWrapper
            AccountMeta::new_readonly(*spl_account_compression_program_id.key, false),  // compressionProgram 
            AccountMeta::new_readonly(*system_program_id.key, false),  // systemProgram
        ];
        
        let mut data: Vec<u8> = vec![];
        data.extend(BURN_DICRIMINATOR);
        data.extend(burn_cnft_data.root.as_ref());
        data.extend(burn_cnft_data.data_hash.as_ref());
        data.extend(burn_cnft_data.creator_hash.as_ref());
        data.extend(burn_cnft_data.nonce.to_le_bytes());
        data.extend(burn_cnft_data.nonce.to_le_bytes());
        
        let mut account_infos: Vec<AccountInfo> = vec![
            mpl_bubblegum_program_id.clone(),
            tree_authority_pda.clone(),
            signer.clone(),
            leaf_delegate.clone(),
            merkle_tree.clone(),
            log_wrapper_program_id.clone(),
            spl_account_compression_program_id.clone(),
            system_program_id.clone(),
        ];

        for _n in 0..burn_cnft_data.proof_length {
            let acct = next_account_info(accounts_iter)?;
            accounts.push(AccountMeta::new_readonly(*acct.key, false));
            account_infos.push(acct.clone());
        }
        
        invoke(
            &solana_program::instruction::Instruction {
                program_id: mpl_bubblegum::ID,
                accounts,   
                data,
            },
            &account_infos[..],
        )?;
        
        Ok(())
    }
}