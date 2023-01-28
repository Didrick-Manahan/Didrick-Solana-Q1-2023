/*Program Logic, where the magic happens! */

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
}; //we saw this also in entrypoint.rs

use crate::{error::EscrowError, instruction::EscrowInstruction, state::Escrow}; //getting from instruction.rs
use spl_token::state::Account as TokenAccount;

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
            EscrowInstruction::Exchange { amount } => {
                msg!("Instruction: Exchange");
                Self::process_exchange(accounts, amount, program_id)
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

        //Created the escrow struct instance and check it is uninitialized.
        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?; //unpack_unchecked -> function inside state.rs. We never defined it but traits can have default functions that may be overridden but don't have to be!
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        //Time to populate the struct's fields!
        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;

        //pack is another default function which internally calls our pack_into_slice function.
        Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;

        //Program Derived Addresses do not lie on the ed25519 curve and therefore have no private key associated with them. */
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

        //transfer authority of the temporary token account to the PDA (derived from escrow program)
        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            //token program helper function "set authority"
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;

        //Cross-Program Invocation!
        msg!("Calling the token program to transfer token account ownership...");

        //program being called through a CPI must be included as an account in the 2nd argument of invoke (and invoke_signed)
        //check to make sure token program is truly the account of the token program --> spl-token versions above 3.1.1 checks for you!
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;
        //Signature Extension! --> the signature is extended to the CPIs.

        fn process_exchange(
            accounts: &[AccountInfo],
            amount_expected_by_taker: u64,
            program_id: &Pubkey,
        ) -> ProgramResult {
            let account_info_iter = &mut accounts.iter();
            let taker = next_account_info(account_info_iter)?;

            //we just get the accounts and do some checks on them below, verifying that Bob has actually passed in the correct accounts with the correct values

            if !taker.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let takers_sending_token_account = next_account_info(account_info_iter)?;

            let takers_token_to_receive_account = next_account_info(account_info_iter)?;

            let pdas_temp_token_account = next_account_info(account_info_iter)?;
            let pdas_temp_token_account_info =
                TokenAccount::unpack(&pdas_temp_token_account.try_borrow_data()?)?;
            let (pda, bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

            //check the amount in the PDA's X token account is what Bob expects
            if amount_expected_by_taker != pdas_temp_token_account_info.amount {
                return Err(EscrowError::ExpectedAmountMismatch.into());
            }

            let initializers_main_account = next_account_info(account_info_iter)?;
            let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
            let escrow_account = next_account_info(account_info_iter)?;

            let escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;

            if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
                return Err(ProgramError::InvalidAccountData);
            }

            if escrow_info.initializer_pubkey != *initializers_main_account.key {
                return Err(ProgramError::InvalidAccountData);
            }

            if escrow_info.initializer_token_to_receive_account_pubkey
                != *initializers_token_to_receive_account.key
            {
                return Err(ProgramError::InvalidAccountData);
            }

            let token_program = next_account_info(account_info_iter)?;

            //transfer tokens!
            let transfer_to_initializer_ix = spl_token::instruction::transfer(
                token_program.key,
                takers_sending_token_account.key,
                initializers_token_to_receive_account.key,
                taker.key,
                &[&taker.key],
                escrow_info.expected_amount,
            )?;
            msg!("Calling the token program to transfer tokens to the escrow's initializer...");
            invoke(
                &transfer_to_initializer_ix,
                &[
                    takers_sending_token_account.clone(),
                    initializers_token_to_receive_account.clone(),
                    taker.clone(),
                    token_program.clone(),
                ],
            )?;
            Ok(())
        }

        Ok(())
    }
}
