/*Entry to the program*/

//smart contracts on Solana are called "Programs"
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
}; //required crates

entrypoint!(process_instruction); //all accounts read or written to must be passed into entrypoint (runtime will parallelize transactions)
fn process_instruction(
    program_id: &Pubkey,      //program ID of currently executing program
    accounts: &[AccountInfo], //used to store state, programs are stateless. Accounts are owned by programs! Only account owner may debit, anyone else can credit!
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Ok(())
}
