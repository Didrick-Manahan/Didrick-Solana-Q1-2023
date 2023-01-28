/*Program Logic, where the magic happens! */

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
}; //we saw this also in entrypoint.rs

use crate::{error::EscrowError, instruction::EscrowInstruction, state::Escrow}; //getting from instruction.rs

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?; //reference to slice holding instruction_data (from entrypoint.rs) goes into the unpack functinon (instruction.rs)

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter(); //mutable so we can take elements out of it
        let initializer = next_account_info(account_info_iter)?; //first account we expect is escrow's initializer

        //Alice needs to be a signer --> boolean on AccountInfo
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        //no need to check if owned by token program, since we will be transfering ownership to PDA (ownership transfer will automatically fail if not owned by token program. No check explicity needed!)
        let temp_token_account = next_account_info(account_info_iter)?; //needs to be writable but no need to check, transaction will fail automatically

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            //check that this is actually owned by the token program (changes aren't being made, so we need explicit check). If we didnt have this check, instead of Alice's transaction failing, Bob's would fail!
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        //checking if account is rent exempt
        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?; //unpack_unchecked -> function inside state.rs. We never defined it but traits can have default functions that may be overridden but don't have to be!
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(())
    }
}
