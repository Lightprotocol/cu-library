use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::pinocchio_ops::msg::msg10_chars;
pub mod pinocchio_ops;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum CuLibraryInstruction {
    Msg10 = 1,
}

impl From<CuLibraryInstruction> for Vec<u8> {
    fn from(value: CuLibraryInstruction) -> Self {
        (value as u16).to_le_bytes().to_vec()
    }
}

impl TryFrom<&[u8]> for CuLibraryInstruction {
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let discriminator = u16::from_le_bytes([value[0], value[1]]);
        match discriminator {
            1 => Ok(CuLibraryInstruction::Msg10),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

#[cfg(not(feature = "no-entrypoint"))]
use pinocchio::{program_entrypoint, default_panic_handler};

#[cfg(not(feature = "no-entrypoint"))]
program_entrypoint!(process_instruction);

#[cfg(not(feature = "no-entrypoint"))]
default_panic_handler!();

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CuLibraryInstruction::try_from(instruction_data)?;
    match instruction {
        CuLibraryInstruction::Msg10 => msg10_chars(),
    }
}
