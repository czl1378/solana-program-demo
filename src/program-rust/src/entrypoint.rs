use byteorder::{ByteOrder, LittleEndian};

use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint, 
    entrypoint::ProgramResult,
    info,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::str::from_utf8;

use std::mem;

entrypoint!(process_instruction);

// Program entrypoint's implementation
fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // A number to store
) -> ProgramResult {
    info!("Demo program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        info!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // The data must be large enough to hold a u64 count
    if account.try_data_len()? < mem::size_of::<u32>() {
        info!("Account data length too small for u32");
        return Err(ProgramError::InvalidAccountData);
    }

    // Store the number
    let mut data = account.try_borrow_mut_data()?;
   
    let num_str = from_utf8(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;
    let num = num_str.to_string().parse::<u32>().unwrap();

    LittleEndian::write_u32(&mut data[0..], num);

    info!("Stored number success!");
    Ok(())
}