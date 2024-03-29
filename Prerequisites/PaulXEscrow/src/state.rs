/*program objects, (de)serializes state

- Responsible for:
    1) defining state objects that the processor can use
    2) serializing and deserializing such objects from and into arrays of u8 respectively.
*/
use solana_program::{
    clock::Clock,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
    pub is_initialized: bool, //determine whether a given escrow account already in use
    pub initializer_pubkey: Pubkey,

    //when Bob takes trade, escrow program sends tokens from `temp_token_account_pubkey` account to Bob's account.
    //Also security check to make sure Bob doesnt pass different token account.
    pub temp_token_account_pubkey: Pubkey,

    //When Bob takes the trade, his tokens will be sent to this account
    pub initializer_token_to_receive_account_pubkey: Pubkey,

    //Used to make sure Bob send's enough of his tokens (no cheating!)
    pub expected_amount: u64,

    //unlock and lock times
    pub unlock_time: u64,
    pub time_out: u64,
}

impl Sealed for Escrow {} //Solana's version of Rust's Sized trait

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 121;

    //DESERIALIZATION OF STATE
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        // unpack_from_slice --> static constructor function returning a new instance of an escrow struct (don't have access to self yet)
        //turns an array of u8 into an instance of the Escrow struct we defined above, uses array_ref (references to sections of an array)
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
            unlock_time,
            time_out,
        ) = array_refs![src, 1, 32, 32, 32, 8, 8, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(
                *initializer_token_to_receive_account_pubkey,
            ),
            expected_amount: u64::from_le_bytes(*expected_amount),
            time_out: 100,
            unlock_time: 1000,
        })
    }

    //SERIALIZATION OF STATE (here was pass in a reference to the Escrow struct)
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
            unlock_time,
            time_out,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 8];

        let Escrow {
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
            time_out,
            unlock_time,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst
            .copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
    }
}

//Any account may be passed into the entry point! It's the program's responsibility to check that received accounts == expected accounts !!
