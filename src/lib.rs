use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use light_program_profiler::profile;

pub mod collections;
pub mod conversions;
pub mod instructions;
pub mod math;
pub mod option;
pub mod partial_eq;
pub mod pinocchio_crates;
pub mod serialization;
pub mod solana_crates;
pub use crate::instructions::discriminator::CuLibraryInstruction;

#[profile]
pub fn baseline_empty_function() {}

#[cfg(not(feature = "no-entrypoint"))]
use pinocchio::{default_panic_handler, program_entrypoint};

use crate::instructions::{
    process_instruction_0_99, process_instruction_100_199, process_instruction_200_299,
};

#[cfg(not(feature = "no-entrypoint"))]
program_entrypoint!(process_instruction);

#[cfg(not(feature = "no-entrypoint"))]
default_panic_handler!();
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let instruction = CuLibraryInstruction::try_from(instruction_data)?;

    // Route to helper functions based on discriminator to reduce stack usage
    let discriminator = u16::from_le_bytes([instruction_data[0], instruction_data[1]]);
    if discriminator < 100 {
        return process_instruction_0_99(instruction, program_id, accounts, instruction_data);
    } else if discriminator < 200 {
        return process_instruction_100_199(instruction, program_id, accounts, instruction_data);
    } else {
        return process_instruction_200_299(instruction, program_id, accounts, instruction_data);
    }
}
