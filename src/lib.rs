use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::pinocchio_ops::msg::pinocchio_msg10_chars;
use crate::pinocchio_ops::sysvar_clock::pinocchio_clock_get_slot;
use crate::pinocchio_ops::sysvar_rent::pinocchio_sysvar_rent_exemption_165;
use crate::solana_ops::msg::solana_msg10_chars;
use crate::solana_ops::msg_program_id::solana_msg_program_id;
use crate::solana_ops::pubkey_new_from_array::solana_pubkey_new_from_array;
use crate::array_vec::vec_new::array_vec_new;
use crate::array_vec::vec_with_capacity::{array_vec_with_capacity_10, array_vec_with_capacity_100};
use crate::array_vec::vec_push::{array_vec_push_u8, array_vec_push_u64, array_vec_push_pubkey};
pub mod pinocchio_ops;
pub mod solana_ops;
pub mod array_vec;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum CuLibraryInstruction {
    Msg10 = 1,
    SolanaMsg10 = 2,
    SolanaMsgProgramId = 3,
    SolanaPubkeyNewFromArray = 4,
    PinocchioSysvarRentExemption165 = 5,
    PinocchioClockGetSlot = 6,
    ArrayVecNew = 7,
    ArrayVecWithCapacity10 = 8,
    ArrayVecWithCapacity100 = 9,
    ArrayVecPushU8 = 10,
    ArrayVecPushU64 = 11,
    ArrayVecPushPubkey = 12,
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
            2 => Ok(CuLibraryInstruction::SolanaMsg10),
            3 => Ok(CuLibraryInstruction::SolanaMsgProgramId),
            4 => Ok(CuLibraryInstruction::SolanaPubkeyNewFromArray),
            5 => Ok(CuLibraryInstruction::PinocchioSysvarRentExemption165),
            6 => Ok(CuLibraryInstruction::PinocchioClockGetSlot),
            7 => Ok(CuLibraryInstruction::ArrayVecNew),
            8 => Ok(CuLibraryInstruction::ArrayVecWithCapacity10),
            9 => Ok(CuLibraryInstruction::ArrayVecWithCapacity100),
            10 => Ok(CuLibraryInstruction::ArrayVecPushU8),
            11 => Ok(CuLibraryInstruction::ArrayVecPushU64),
            12 => Ok(CuLibraryInstruction::ArrayVecPushPubkey),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

#[cfg(not(feature = "no-entrypoint"))]
use pinocchio::{default_panic_handler, program_entrypoint};

#[cfg(not(feature = "no-entrypoint"))]
program_entrypoint!(process_instruction);

#[cfg(not(feature = "no-entrypoint"))]
default_panic_handler!();

pub fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CuLibraryInstruction::try_from(instruction_data)?;
    match instruction {
        CuLibraryInstruction::Msg10 => pinocchio_msg10_chars(),
        CuLibraryInstruction::SolanaMsg10 => solana_msg10_chars(),
        CuLibraryInstruction::SolanaMsgProgramId => solana_msg_program_id(program_id),
        CuLibraryInstruction::SolanaPubkeyNewFromArray => solana_pubkey_new_from_array(program_id),
        CuLibraryInstruction::PinocchioSysvarRentExemption165 => pinocchio_sysvar_rent_exemption_165(),
        CuLibraryInstruction::PinocchioClockGetSlot => pinocchio_clock_get_slot(),
        CuLibraryInstruction::ArrayVecNew => array_vec_new(),
        CuLibraryInstruction::ArrayVecWithCapacity10 => array_vec_with_capacity_10(),
        CuLibraryInstruction::ArrayVecWithCapacity100 => array_vec_with_capacity_100(),
        CuLibraryInstruction::ArrayVecPushU8 => array_vec_push_u8(),
        CuLibraryInstruction::ArrayVecPushU64 => array_vec_push_u64(),
        CuLibraryInstruction::ArrayVecPushPubkey => array_vec_push_pubkey(program_id),
    }
}
