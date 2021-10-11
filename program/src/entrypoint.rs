//! Program entrypoint

use crate::processor::Processor;

use solana_program::{
    msg,
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

// Program entrypoint's implementation
fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("calling proccess!");
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        // error.print::<Error>();
        // msg!("{:?}", error);
        return Err(error);
    }
    Ok(())
}
