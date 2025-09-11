use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::array::array_assign::{
    array_assign_10_pubkey, array_assign_10_u64, array_assign_10_u8, array_assign_pubkey,
    array_assign_u64, array_assign_u8,
};
use crate::array::array_new::array_new;
use crate::array::array_with_capacity::{array_with_capacity_10, array_with_capacity_100};
use crate::arrayvec::vec_new::arrayvec_u8_new;
use crate::arrayvec::vec_push::{
    arrayvec_push_10_pubkey, arrayvec_push_10_u64, arrayvec_push_10_u8, arrayvec_push_pubkey,
    arrayvec_push_u64, arrayvec_push_u8,
};
use crate::arrayvec::vec_with_capacity::{
    arrayvec_u8_with_capacity_10, arrayvec_u8_with_capacity_100,
};
use crate::checked_math::checked_add::{
    checked_add_u128, checked_add_u16, checked_add_u32, checked_add_u64, checked_add_u8,
};
use crate::checked_math::checked_div::{
    checked_div_u128, checked_div_u16, checked_div_u32, checked_div_u64, checked_div_u8,
};
use crate::checked_math::checked_mul::{
    checked_mul_u128, checked_mul_u16, checked_mul_u32, checked_mul_u64, checked_mul_u8,
};
use crate::checked_math::checked_sub::{
    checked_sub_u128, checked_sub_u16, checked_sub_u32, checked_sub_u64, checked_sub_u8,
};
use crate::pinocchio_ops::msg::pinocchio_msg10_chars;
use crate::pinocchio_ops::sysvar_clock::pinocchio_clock_get_slot;
use crate::pinocchio_ops::sysvar_rent::pinocchio_sysvar_rent_exemption_165;
use crate::saturating_math::saturating_add::{
    saturating_add_u128, saturating_add_u16, saturating_add_u32, saturating_add_u64,
    saturating_add_u8,
};
use crate::saturating_math::saturating_mul::{
    saturating_mul_u128, saturating_mul_u16, saturating_mul_u32, saturating_mul_u64,
    saturating_mul_u8,
};
use crate::saturating_math::saturating_sub::{
    saturating_sub_u128, saturating_sub_u16, saturating_sub_u32, saturating_sub_u64,
    saturating_sub_u8,
};
use crate::solana_ops::msg::solana_msg10_chars;
use crate::solana_ops::msg_program_id::solana_msg_program_id;
use crate::solana_ops::pubkey_new_from_array::solana_pubkey_new_from_array;
use crate::std_math::add_assign::{
    add_assign_u128, add_assign_u16, add_assign_u32, add_assign_u64, add_assign_u8,
};
use crate::std_math::std_add::{std_add_u128, std_add_u16, std_add_u32, std_add_u64, std_add_u8};
use crate::std_math::std_div::{std_div_u128, std_div_u16, std_div_u32, std_div_u64, std_div_u8};
use crate::std_math::std_mul::{std_mul_u128, std_mul_u16, std_mul_u32, std_mul_u64, std_mul_u8};
use crate::std_math::std_sub::{std_sub_u128, std_sub_u16, std_sub_u32, std_sub_u64, std_sub_u8};
use crate::std_math::sub_assign::{
    sub_assign_u128, sub_assign_u16, sub_assign_u32, sub_assign_u64, sub_assign_u8,
};
use crate::vec::vec_new::vec_u8_new;
use crate::vec::vec_push::{
    vec_push_10_pubkey, vec_push_10_u64, vec_push_10_u8, vec_push_pubkey, vec_push_u64, vec_push_u8,
    vec_push_u8_with_capacity, vec_push_u64_with_capacity, vec_push_pubkey_with_capacity,
    vec_push_10_u8_with_capacity, vec_push_10_u64_with_capacity, vec_push_10_pubkey_with_capacity,
};
use crate::vec::vec_with_capacity::{vec_u8_with_capacity_10, vec_u8_with_capacity_100};
use light_program_profiler::profile;

pub mod array;
pub mod arrayvec;
pub mod checked_math;
pub mod pinocchio_ops;
pub mod saturating_math;
pub mod solana_ops;
pub mod std_math;
pub mod vec;

#[profile]
pub fn baseline_empty_function() {}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum CuLibraryInstruction {
    Baseline = 0,
    Msg10 = 1,
    SolanaMsg10 = 2,
    SolanaMsgProgramId = 3,
    SolanaPubkeyNewFromArray = 4,
    PinocchioSysvarRentExemption165 = 5,
    PinocchioClockGetSlot = 6,
    ArrayvecNew = 7,
    ArrayvecWithCapacity10 = 8,
    ArrayvecWithCapacity100 = 9,
    ArrayvecPushU8 = 10,
    ArrayvecPushU64 = 11,
    ArrayvecPushPubkey = 12,
    ArrayvecPush10U8 = 13,
    ArrayvecPush10U64 = 14,
    ArrayvecPush10Pubkey = 15,
    VecNew = 16,
    VecWithCapacity10 = 17,
    VecWithCapacity100 = 18,
    VecPushU8 = 19,
    VecPushU64 = 20,
    VecPushPubkey = 21,
    VecPush10U8 = 22,
    VecPush10U64 = 23,
    VecPush10Pubkey = 24,
    ArrayNew = 25,
    ArrayWithCapacity10 = 26,
    ArrayWithCapacity100 = 27,
    ArrayAssignU8 = 28,
    ArrayAssignU64 = 29,
    ArrayAssignPubkey = 30,
    ArrayAssign10U8 = 31,
    ArrayAssign10U64 = 32,
    ArrayAssign10Pubkey = 33,
    CheckedAddU8 = 34,
    CheckedAddU16 = 35,
    CheckedAddU32 = 36,
    CheckedAddU64 = 37,
    CheckedAddU128 = 38,
    CheckedSubU8 = 39,
    CheckedSubU16 = 40,
    CheckedSubU32 = 41,
    CheckedSubU64 = 42,
    CheckedSubU128 = 43,
    CheckedMulU8 = 44,
    CheckedMulU16 = 45,
    CheckedMulU32 = 46,
    CheckedMulU64 = 47,
    CheckedMulU128 = 48,
    CheckedDivU8 = 49,
    CheckedDivU16 = 50,
    CheckedDivU32 = 51,
    CheckedDivU64 = 52,
    CheckedDivU128 = 53,
    SaturatingAddU8 = 54,
    SaturatingAddU16 = 55,
    SaturatingAddU32 = 56,
    SaturatingAddU64 = 57,
    SaturatingAddU128 = 58,
    SaturatingSubU8 = 59,
    SaturatingSubU16 = 60,
    SaturatingSubU32 = 61,
    SaturatingSubU64 = 62,
    SaturatingSubU128 = 63,
    SaturatingMulU8 = 64,
    SaturatingMulU16 = 65,
    SaturatingMulU32 = 66,
    SaturatingMulU64 = 67,
    SaturatingMulU128 = 68,
    StdAddU8 = 69,
    StdAddU16 = 70,
    StdAddU32 = 71,
    StdAddU64 = 72,
    StdAddU128 = 73,
    StdSubU8 = 74,
    StdSubU16 = 75,
    StdSubU32 = 76,
    StdSubU64 = 77,
    StdSubU128 = 78,
    StdMulU8 = 79,
    StdMulU16 = 80,
    StdMulU32 = 81,
    StdMulU64 = 82,
    StdMulU128 = 83,
    StdDivU8 = 84,
    StdDivU16 = 85,
    StdDivU32 = 86,
    StdDivU64 = 87,
    StdDivU128 = 88,
    AddAssignU8 = 89,
    AddAssignU16 = 90,
    AddAssignU32 = 91,
    AddAssignU64 = 92,
    AddAssignU128 = 93,
    SubAssignU8 = 94,
    SubAssignU16 = 95,
    SubAssignU32 = 96,
    SubAssignU64 = 97,
    SubAssignU128 = 98,
    VecPushU8WithCapacity = 99,
    VecPushU64WithCapacity = 100,
    VecPushPubkeyWithCapacity = 101,
    VecPush10U8WithCapacity = 102,
    VecPush10U64WithCapacity = 103,
    VecPush10PubkeyWithCapacity = 104,
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
            0 => Ok(CuLibraryInstruction::Baseline),
            1 => Ok(CuLibraryInstruction::Msg10),
            2 => Ok(CuLibraryInstruction::SolanaMsg10),
            3 => Ok(CuLibraryInstruction::SolanaMsgProgramId),
            4 => Ok(CuLibraryInstruction::SolanaPubkeyNewFromArray),
            5 => Ok(CuLibraryInstruction::PinocchioSysvarRentExemption165),
            6 => Ok(CuLibraryInstruction::PinocchioClockGetSlot),
            7 => Ok(CuLibraryInstruction::ArrayvecNew),
            8 => Ok(CuLibraryInstruction::ArrayvecWithCapacity10),
            9 => Ok(CuLibraryInstruction::ArrayvecWithCapacity100),
            10 => Ok(CuLibraryInstruction::ArrayvecPushU8),
            11 => Ok(CuLibraryInstruction::ArrayvecPushU64),
            12 => Ok(CuLibraryInstruction::ArrayvecPushPubkey),
            13 => Ok(CuLibraryInstruction::ArrayvecPush10U8),
            14 => Ok(CuLibraryInstruction::ArrayvecPush10U64),
            15 => Ok(CuLibraryInstruction::ArrayvecPush10Pubkey),
            16 => Ok(CuLibraryInstruction::VecNew),
            17 => Ok(CuLibraryInstruction::VecWithCapacity10),
            18 => Ok(CuLibraryInstruction::VecWithCapacity100),
            19 => Ok(CuLibraryInstruction::VecPushU8),
            20 => Ok(CuLibraryInstruction::VecPushU64),
            21 => Ok(CuLibraryInstruction::VecPushPubkey),
            22 => Ok(CuLibraryInstruction::VecPush10U8),
            23 => Ok(CuLibraryInstruction::VecPush10U64),
            24 => Ok(CuLibraryInstruction::VecPush10Pubkey),
            25 => Ok(CuLibraryInstruction::ArrayNew),
            26 => Ok(CuLibraryInstruction::ArrayWithCapacity10),
            27 => Ok(CuLibraryInstruction::ArrayWithCapacity100),
            28 => Ok(CuLibraryInstruction::ArrayAssignU8),
            29 => Ok(CuLibraryInstruction::ArrayAssignU64),
            30 => Ok(CuLibraryInstruction::ArrayAssignPubkey),
            31 => Ok(CuLibraryInstruction::ArrayAssign10U8),
            32 => Ok(CuLibraryInstruction::ArrayAssign10U64),
            33 => Ok(CuLibraryInstruction::ArrayAssign10Pubkey),
            34 => Ok(CuLibraryInstruction::CheckedAddU8),
            35 => Ok(CuLibraryInstruction::CheckedAddU16),
            36 => Ok(CuLibraryInstruction::CheckedAddU32),
            37 => Ok(CuLibraryInstruction::CheckedAddU64),
            38 => Ok(CuLibraryInstruction::CheckedAddU128),
            39 => Ok(CuLibraryInstruction::CheckedSubU8),
            40 => Ok(CuLibraryInstruction::CheckedSubU16),
            41 => Ok(CuLibraryInstruction::CheckedSubU32),
            42 => Ok(CuLibraryInstruction::CheckedSubU64),
            43 => Ok(CuLibraryInstruction::CheckedSubU128),
            44 => Ok(CuLibraryInstruction::CheckedMulU8),
            45 => Ok(CuLibraryInstruction::CheckedMulU16),
            46 => Ok(CuLibraryInstruction::CheckedMulU32),
            47 => Ok(CuLibraryInstruction::CheckedMulU64),
            48 => Ok(CuLibraryInstruction::CheckedMulU128),
            49 => Ok(CuLibraryInstruction::CheckedDivU8),
            50 => Ok(CuLibraryInstruction::CheckedDivU16),
            51 => Ok(CuLibraryInstruction::CheckedDivU32),
            52 => Ok(CuLibraryInstruction::CheckedDivU64),
            53 => Ok(CuLibraryInstruction::CheckedDivU128),
            54 => Ok(CuLibraryInstruction::SaturatingAddU8),
            55 => Ok(CuLibraryInstruction::SaturatingAddU16),
            56 => Ok(CuLibraryInstruction::SaturatingAddU32),
            57 => Ok(CuLibraryInstruction::SaturatingAddU64),
            58 => Ok(CuLibraryInstruction::SaturatingAddU128),
            59 => Ok(CuLibraryInstruction::SaturatingSubU8),
            60 => Ok(CuLibraryInstruction::SaturatingSubU16),
            61 => Ok(CuLibraryInstruction::SaturatingSubU32),
            62 => Ok(CuLibraryInstruction::SaturatingSubU64),
            63 => Ok(CuLibraryInstruction::SaturatingSubU128),
            64 => Ok(CuLibraryInstruction::SaturatingMulU8),
            65 => Ok(CuLibraryInstruction::SaturatingMulU16),
            66 => Ok(CuLibraryInstruction::SaturatingMulU32),
            67 => Ok(CuLibraryInstruction::SaturatingMulU64),
            68 => Ok(CuLibraryInstruction::SaturatingMulU128),
            69 => Ok(CuLibraryInstruction::StdAddU8),
            70 => Ok(CuLibraryInstruction::StdAddU16),
            71 => Ok(CuLibraryInstruction::StdAddU32),
            72 => Ok(CuLibraryInstruction::StdAddU64),
            73 => Ok(CuLibraryInstruction::StdAddU128),
            74 => Ok(CuLibraryInstruction::StdSubU8),
            75 => Ok(CuLibraryInstruction::StdSubU16),
            76 => Ok(CuLibraryInstruction::StdSubU32),
            77 => Ok(CuLibraryInstruction::StdSubU64),
            78 => Ok(CuLibraryInstruction::StdSubU128),
            79 => Ok(CuLibraryInstruction::StdMulU8),
            80 => Ok(CuLibraryInstruction::StdMulU16),
            81 => Ok(CuLibraryInstruction::StdMulU32),
            82 => Ok(CuLibraryInstruction::StdMulU64),
            83 => Ok(CuLibraryInstruction::StdMulU128),
            84 => Ok(CuLibraryInstruction::StdDivU8),
            85 => Ok(CuLibraryInstruction::StdDivU16),
            86 => Ok(CuLibraryInstruction::StdDivU32),
            87 => Ok(CuLibraryInstruction::StdDivU64),
            88 => Ok(CuLibraryInstruction::StdDivU128),
            89 => Ok(CuLibraryInstruction::AddAssignU8),
            90 => Ok(CuLibraryInstruction::AddAssignU16),
            91 => Ok(CuLibraryInstruction::AddAssignU32),
            92 => Ok(CuLibraryInstruction::AddAssignU64),
            93 => Ok(CuLibraryInstruction::AddAssignU128),
            94 => Ok(CuLibraryInstruction::SubAssignU8),
            95 => Ok(CuLibraryInstruction::SubAssignU16),
            96 => Ok(CuLibraryInstruction::SubAssignU32),
            97 => Ok(CuLibraryInstruction::SubAssignU64),
            98 => Ok(CuLibraryInstruction::SubAssignU128),
            99 => Ok(CuLibraryInstruction::VecPushU8WithCapacity),
            100 => Ok(CuLibraryInstruction::VecPushU64WithCapacity),
            101 => Ok(CuLibraryInstruction::VecPushPubkeyWithCapacity),
            102 => Ok(CuLibraryInstruction::VecPush10U8WithCapacity),
            103 => Ok(CuLibraryInstruction::VecPush10U64WithCapacity),
            104 => Ok(CuLibraryInstruction::VecPush10PubkeyWithCapacity),
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
) -> Result<(), ProgramError> {
    let instruction = CuLibraryInstruction::try_from(instruction_data)?;
    match instruction {
        CuLibraryInstruction::Baseline => {
            baseline_empty_function();
            solana_msg::msg!("baseline complete");
        }
        CuLibraryInstruction::Msg10 => pinocchio_msg10_chars()?,
        CuLibraryInstruction::SolanaMsg10 => solana_msg10_chars()?,
        CuLibraryInstruction::SolanaMsgProgramId => solana_msg_program_id(program_id)?,
        CuLibraryInstruction::SolanaPubkeyNewFromArray => {
            let res = solana_pubkey_new_from_array(program_id);
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::PinocchioSysvarRentExemption165 => {
            let _ = pinocchio_sysvar_rent_exemption_165();
        }
        CuLibraryInstruction::PinocchioClockGetSlot => pinocchio_clock_get_slot()?,
        CuLibraryInstruction::ArrayvecNew => {
            let res = arrayvec_u8_new();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecWithCapacity10 => {
            let res = arrayvec_u8_with_capacity_10();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecWithCapacity100 => {
            let res = arrayvec_u8_with_capacity_100();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecPushU8 => {
            let res = arrayvec_push_u8();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecPushU64 => {
            let res = arrayvec_push_u64();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecPushPubkey => {
            let res = arrayvec_push_pubkey(program_id);
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecPush10U8 => {
            let res = arrayvec_push_10_u8();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecPush10U64 => {
            let res = arrayvec_push_10_u64();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecPush10Pubkey => {
            let res = arrayvec_push_10_pubkey(program_id)?;
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::VecNew => {
            let res = vec_u8_new();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecWithCapacity10 => {
            let res = vec_u8_with_capacity_10();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecWithCapacity100 => {
            let res = vec_u8_with_capacity_100();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushU8 => {
            let res = vec_push_u8();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushU64 => {
            let res = vec_push_u64();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushPubkey => {
            let res = vec_push_pubkey(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U8 => {
            let res = vec_push_10_u8();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U64 => {
            let res = vec_push_10_u64();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10Pubkey => {
            let res = vec_push_10_pubkey(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::ArrayNew => {
            let res = array_new();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayWithCapacity10 => {
            let res = array_with_capacity_10();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayWithCapacity100 => {
            let res = array_with_capacity_100();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssignU8 => {
            let res = array_assign_u8();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssignU64 => {
            let res = array_assign_u64();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssignPubkey => {
            let res = array_assign_pubkey(program_id);
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssign10U8 => {
            let res = array_assign_10_u8();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssign10U64 => {
            let res = array_assign_10_u64();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssign10Pubkey => {
            let res = array_assign_10_pubkey(program_id);
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU8 => {
            let res = checked_add_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU16 => {
            let res = checked_add_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU32 => {
            let res = checked_add_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU64 => {
            let res = checked_add_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU128 => {
            let res = checked_add_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU8 => {
            let res = checked_sub_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU16 => {
            let res = checked_sub_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU32 => {
            let res = checked_sub_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU64 => {
            let res = checked_sub_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU128 => {
            let res = checked_sub_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU8 => {
            let res = checked_mul_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU16 => {
            let res = checked_mul_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU32 => {
            let res = checked_mul_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU64 => {
            let res = checked_mul_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU128 => {
            let res = checked_mul_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU8 => {
            let res = checked_div_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU16 => {
            let res = checked_div_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU32 => {
            let res = checked_div_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU64 => {
            let res = checked_div_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU128 => {
            let res = checked_div_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU8 => {
            let res = saturating_add_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU16 => {
            let res = saturating_add_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU32 => {
            let res = saturating_add_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU64 => {
            let res = saturating_add_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU128 => {
            let res = saturating_add_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU8 => {
            let res = saturating_sub_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU16 => {
            let res = saturating_sub_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU32 => {
            let res = saturating_sub_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU64 => {
            let res = saturating_sub_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU128 => {
            let res = saturating_sub_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU8 => {
            let res = saturating_mul_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU16 => {
            let res = saturating_mul_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU32 => {
            let res = saturating_mul_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU64 => {
            let res = saturating_mul_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU128 => {
            let res = saturating_mul_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU8 => {
            let res = std_add_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU16 => {
            let res = std_add_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU32 => {
            let res = std_add_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU64 => {
            let res = std_add_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU128 => {
            let res = std_add_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU8 => {
            let res = std_sub_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU16 => {
            let res = std_sub_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU32 => {
            let res = std_sub_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU64 => {
            let res = std_sub_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU128 => {
            let res = std_sub_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU8 => {
            let res = std_mul_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU16 => {
            let res = std_mul_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU32 => {
            let res = std_mul_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU64 => {
            let res = std_mul_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU128 => {
            let res = std_mul_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU8 => {
            let res = std_div_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU16 => {
            let res = std_div_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU32 => {
            let res = std_div_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU64 => {
            let res = std_div_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU128 => {
            let res = std_div_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::AddAssignU8 => {
            let res = add_assign_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::AddAssignU16 => {
            let res = add_assign_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::AddAssignU32 => {
            let res = add_assign_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::AddAssignU64 => {
            let res = add_assign_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::AddAssignU128 => {
            let res = add_assign_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SubAssignU8 => {
            let res = sub_assign_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SubAssignU16 => {
            let res = sub_assign_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SubAssignU32 => {
            let res = sub_assign_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SubAssignU64 => {
            let res = sub_assign_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SubAssignU128 => {
            let res = sub_assign_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::VecPushU8WithCapacity => {
            let res = vec_push_u8_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushU64WithCapacity => {
            let res = vec_push_u64_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushPubkeyWithCapacity => {
            let res = vec_push_pubkey_with_capacity(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U8WithCapacity => {
            let res = vec_push_10_u8_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U64WithCapacity => {
            let res = vec_push_10_u64_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10PubkeyWithCapacity => {
            let res = vec_push_10_pubkey_with_capacity(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
    }
    Ok(())
}
