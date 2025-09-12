use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::account_info::account_borrows::{
    account_info_borrow_data_unchecked, account_info_borrow_lamports_unchecked,
    account_info_borrow_mut_data_unchecked, account_info_borrow_mut_lamports_unchecked,
    account_info_can_borrow_data, account_info_can_borrow_lamports,
    account_info_can_borrow_mut_data, account_info_can_borrow_mut_lamports,
    account_info_is_borrowed, account_info_try_borrow_data, account_info_try_borrow_lamports,
    account_info_try_borrow_mut_data, account_info_try_borrow_mut_lamports,
};
use crate::account_info::account_checks::{
    account_info_data_is_empty, account_info_executable, account_info_is_signer,
    account_info_is_writable,
};
use crate::account_info::account_data::{account_info_data_len, account_info_lamports};
use crate::account_info::account_key::account_info_key;
use crate::account_info::account_owner::account_info_owner;
use crate::account_info::account_ownership::{account_info_assign, account_info_is_owned_by};
use crate::account_info::account_realloc::{
    account_info_close, account_info_close_unchecked, account_info_realloc,
};
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
use crate::conversions::cast_u16::{
    conversions_u16_as_u32, conversions_u16_as_u64,
    conversions_u16_as_u8, conversions_u16_as_usize,
};
use crate::conversions::cast_u32::{
    conversions_u32_as_u16, conversions_u32_as_u64,
    conversions_u32_as_u8, conversions_u32_as_usize,
};
use crate::conversions::cast_u64::{
    conversions_u64_as_u16, conversions_u64_as_u32,
    conversions_u64_as_u8, conversions_u64_as_usize,
};
use crate::conversions::cast_u8::{
    conversions_u8_as_u16, conversions_u8_as_u32, conversions_u8_as_u64,
    conversions_u8_as_usize,
};
use crate::conversions::from_usize::{
    conversions_try_into_usize_to_u64_map_err, conversions_try_into_usize_to_u64_unwrap,
};
use crate::conversions::slice_to_array::{
    conversions_try_into_slice_to_array_32_map_err,
    conversions_try_into_slice_to_array_32_unwrap,
};
use crate::conversions::to_usize::{
    conversions_try_into_u16_to_usize_map_err,
    conversions_try_into_u16_to_usize_unwrap,
    conversions_try_into_u32_to_usize_map_err,
    conversions_try_into_u32_to_usize_unwrap,
    conversions_try_into_u64_to_usize_map_err,
    conversions_try_into_u64_to_usize_unwrap,
    conversions_try_into_u8_to_usize_map_err,
    conversions_try_into_u8_to_usize_unwrap,
};
use crate::cpi::cpi_array_loop::{
    cpi_account_info_array_10_clone_loop, cpi_account_info_array_10_move_loop,
    cpi_account_info_array_10_ref_loop, cpi_account_meta_array_10_loop,
};
use crate::cpi::cpi_arrays::{
    cpi_account_info_array_10_clone, cpi_account_info_array_10_move, cpi_account_info_array_10_ref,
    cpi_account_meta_array_10,
};
use crate::cpi::cpi_arrayvec::{
    cpi_arrayvec_push_account_info_10_clone, cpi_arrayvec_push_account_info_10_move,
    cpi_arrayvec_push_account_info_10_ref, cpi_arrayvec_push_account_meta_10,
};
use crate::partial_eq::partial_eq_arrays::{
    partial_eq_array_u16_32, partial_eq_array_u32_32, partial_eq_array_u64_32,
    partial_eq_array_u8_32, partial_eq_array_u8_32_ref,
};
use crate::partial_eq::partial_eq_neq::{
    partial_eq_array_u16_32_neq, partial_eq_array_u32_32_neq, partial_eq_array_u64_32_neq,
    partial_eq_array_u8_32_neq, partial_eq_array_u8_32_neq_deref, partial_eq_array_u8_32_neq_ref,
    partial_eq_u128_neq, partial_eq_u16_neq, partial_eq_u32_neq, partial_eq_u64_neq,
    partial_eq_u8_neq,
};
use crate::partial_eq::partial_eq_primitives::{
    partial_eq_u128, partial_eq_u16, partial_eq_u32, partial_eq_u64, partial_eq_u8,
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
    vec_push_10_pubkey, vec_push_10_pubkey_with_capacity, vec_push_10_u64,
    vec_push_10_u64_with_capacity, vec_push_10_u8, vec_push_10_u8_with_capacity, vec_push_pubkey,
    vec_push_pubkey_with_capacity, vec_push_u64, vec_push_u64_with_capacity, vec_push_u8,
    vec_push_u8_with_capacity,
};
use crate::vec::vec_with_capacity::{vec_u8_with_capacity_10, vec_u8_with_capacity_100};
use light_program_profiler::profile;

pub mod account_info;
pub mod array;
pub mod arrayvec;
pub mod checked_math;
pub mod conversions;
pub mod cpi;
pub mod partial_eq;
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
    AccountInfoKey = 105,
    AccountInfoOwner = 106,
    AccountInfoIsSigner = 107,
    AccountInfoIsWritable = 108,
    AccountInfoExecutable = 109,
    AccountInfoDataLen = 110,
    AccountInfoLamports = 111,
    AccountInfoDataIsEmpty = 112,
    AccountInfoIsOwnedBy = 113,
    AccountInfoAssign = 114,
    AccountInfoIsBorrowed = 115,
    AccountInfoBorrowLamportsUnchecked = 116,
    AccountInfoBorrowMutLamportsUnchecked = 117,
    AccountInfoBorrowDataUnchecked = 118,
    AccountInfoBorrowMutDataUnchecked = 119,
    AccountInfoTryBorrowLamports = 120,
    AccountInfoTryBorrowMutLamports = 121,
    AccountInfoCanBorrowLamports = 122,
    AccountInfoCanBorrowMutLamports = 123,
    AccountInfoTryBorrowData = 124,
    AccountInfoTryBorrowMutData = 125,
    AccountInfoCanBorrowData = 126,
    AccountInfoCanBorrowMutData = 127,
    AccountInfoRealloc = 128,
    AccountInfoClose = 129,
    AccountInfoCloseUnchecked = 130,
    CpiAccountMetaArray10 = 131,
    CpiAccountInfoArray10Ref = 132,
    CpiAccountInfoArray10Clone = 133,
    CpiAccountInfoArray10Move = 134,
    CpiArrayvecPushAccountMeta10 = 135,
    CpiArrayvecPushAccountInfo10Ref = 136,
    CpiArrayvecPushAccountInfo10Clone = 137,
    CpiArrayvecPushAccountInfo10Move = 138,
    CpiAccountMetaArray10Loop = 139,
    CpiAccountInfoArray10RefLoop = 140,
    CpiAccountInfoArray10CloneLoop = 141,
    CpiAccountInfoArray10MoveLoop = 142,
    PartialEqU8 = 143,
    PartialEqU16 = 144,
    PartialEqU32 = 145,
    PartialEqU64 = 146,
    PartialEqU128 = 147,
    PartialEqArrayU8_32Ref = 148,
    PartialEqArrayU8_32 = 149,
    PartialEqArrayU16_32 = 150,
    PartialEqArrayU32_32 = 151,
    PartialEqArrayU64_32 = 152,
    PartialEqU8Neq = 153,
    PartialEqU16Neq = 154,
    PartialEqU32Neq = 155,
    PartialEqU64Neq = 156,
    PartialEqU128Neq = 157,
    PartialEqArrayU8_32NeqRef = 158,
    PartialEqArrayU8_32Neq = 159,
    PartialEqArrayU8_32NeqDeref = 160,
    PartialEqArrayU16_32Neq = 161,
    PartialEqArrayU32_32Neq = 162,
    PartialEqArrayU64_32Neq = 163,
    // Conversions
    ConversionsSliceToArray32Unwrap = 164,
    ConversionsSliceToArray32MapErr = 165,
    ConversionsUsizeToU64Unwrap = 167,
    ConversionsUsizeToU64MapErr = 168,
    ConversionsU64ToUsizeUnwrap = 170,
    ConversionsU64ToUsizeMapErr = 171,
    ConversionsU32ToUsizeUnwrap = 173,
    ConversionsU32ToUsizeMapErr = 174,
    ConversionsU16ToUsizeUnwrap = 176,
    ConversionsU16ToUsizeMapErr = 177,
    ConversionsU8ToUsizeUnwrap = 179,
    ConversionsU8ToUsizeMapErr = 180,
    // Cast conversions
    ConversionsU8AsU16 = 181,
    ConversionsU8AsU32 = 182,
    ConversionsU8AsU64 = 183,
    ConversionsU8AsUsize = 184,
    ConversionsU16AsU8 = 185,
    ConversionsU16AsU32 = 186,
    ConversionsU16AsU64 = 187,
    ConversionsU16AsUsize = 188,
    ConversionsU32AsU8 = 189,
    ConversionsU32AsU16 = 190,
    ConversionsU32AsU64 = 191,
    ConversionsU32AsUsize = 192,
    ConversionsU64AsU8 = 193,
    ConversionsU64AsU16 = 194,
    ConversionsU64AsU32 = 195,
    ConversionsU64AsUsize = 196,
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
            105 => Ok(CuLibraryInstruction::AccountInfoKey),
            106 => Ok(CuLibraryInstruction::AccountInfoOwner),
            107 => Ok(CuLibraryInstruction::AccountInfoIsSigner),
            108 => Ok(CuLibraryInstruction::AccountInfoIsWritable),
            109 => Ok(CuLibraryInstruction::AccountInfoExecutable),
            110 => Ok(CuLibraryInstruction::AccountInfoDataLen),
            111 => Ok(CuLibraryInstruction::AccountInfoLamports),
            112 => Ok(CuLibraryInstruction::AccountInfoDataIsEmpty),
            113 => Ok(CuLibraryInstruction::AccountInfoIsOwnedBy),
            114 => Ok(CuLibraryInstruction::AccountInfoAssign),
            115 => Ok(CuLibraryInstruction::AccountInfoIsBorrowed),
            116 => Ok(CuLibraryInstruction::AccountInfoBorrowLamportsUnchecked),
            117 => Ok(CuLibraryInstruction::AccountInfoBorrowMutLamportsUnchecked),
            118 => Ok(CuLibraryInstruction::AccountInfoBorrowDataUnchecked),
            119 => Ok(CuLibraryInstruction::AccountInfoBorrowMutDataUnchecked),
            120 => Ok(CuLibraryInstruction::AccountInfoTryBorrowLamports),
            121 => Ok(CuLibraryInstruction::AccountInfoTryBorrowMutLamports),
            122 => Ok(CuLibraryInstruction::AccountInfoCanBorrowLamports),
            123 => Ok(CuLibraryInstruction::AccountInfoCanBorrowMutLamports),
            124 => Ok(CuLibraryInstruction::AccountInfoTryBorrowData),
            125 => Ok(CuLibraryInstruction::AccountInfoTryBorrowMutData),
            126 => Ok(CuLibraryInstruction::AccountInfoCanBorrowData),
            127 => Ok(CuLibraryInstruction::AccountInfoCanBorrowMutData),
            128 => Ok(CuLibraryInstruction::AccountInfoRealloc),
            129 => Ok(CuLibraryInstruction::AccountInfoClose),
            130 => Ok(CuLibraryInstruction::AccountInfoCloseUnchecked),
            131 => Ok(CuLibraryInstruction::CpiAccountMetaArray10),
            132 => Ok(CuLibraryInstruction::CpiAccountInfoArray10Ref),
            133 => Ok(CuLibraryInstruction::CpiAccountInfoArray10Clone),
            134 => Ok(CuLibraryInstruction::CpiAccountInfoArray10Move),
            135 => Ok(CuLibraryInstruction::CpiArrayvecPushAccountMeta10),
            136 => Ok(CuLibraryInstruction::CpiArrayvecPushAccountInfo10Ref),
            137 => Ok(CuLibraryInstruction::CpiArrayvecPushAccountInfo10Clone),
            138 => Ok(CuLibraryInstruction::CpiArrayvecPushAccountInfo10Move),
            139 => Ok(CuLibraryInstruction::CpiAccountMetaArray10Loop),
            140 => Ok(CuLibraryInstruction::CpiAccountInfoArray10RefLoop),
            141 => Ok(CuLibraryInstruction::CpiAccountInfoArray10CloneLoop),
            142 => Ok(CuLibraryInstruction::CpiAccountInfoArray10MoveLoop),
            143 => Ok(CuLibraryInstruction::PartialEqU8),
            144 => Ok(CuLibraryInstruction::PartialEqU16),
            145 => Ok(CuLibraryInstruction::PartialEqU32),
            146 => Ok(CuLibraryInstruction::PartialEqU64),
            147 => Ok(CuLibraryInstruction::PartialEqU128),
            148 => Ok(CuLibraryInstruction::PartialEqArrayU8_32Ref),
            149 => Ok(CuLibraryInstruction::PartialEqArrayU8_32),
            150 => Ok(CuLibraryInstruction::PartialEqArrayU16_32),
            151 => Ok(CuLibraryInstruction::PartialEqArrayU32_32),
            152 => Ok(CuLibraryInstruction::PartialEqArrayU64_32),
            153 => Ok(CuLibraryInstruction::PartialEqU8Neq),
            154 => Ok(CuLibraryInstruction::PartialEqU16Neq),
            155 => Ok(CuLibraryInstruction::PartialEqU32Neq),
            156 => Ok(CuLibraryInstruction::PartialEqU64Neq),
            157 => Ok(CuLibraryInstruction::PartialEqU128Neq),
            158 => Ok(CuLibraryInstruction::PartialEqArrayU8_32NeqRef),
            159 => Ok(CuLibraryInstruction::PartialEqArrayU8_32Neq),
            160 => Ok(CuLibraryInstruction::PartialEqArrayU8_32NeqDeref),
            161 => Ok(CuLibraryInstruction::PartialEqArrayU16_32Neq),
            162 => Ok(CuLibraryInstruction::PartialEqArrayU32_32Neq),
            163 => Ok(CuLibraryInstruction::PartialEqArrayU64_32Neq),
            164 => Ok(CuLibraryInstruction::ConversionsSliceToArray32Unwrap),
            165 => Ok(CuLibraryInstruction::ConversionsSliceToArray32MapErr),
            167 => Ok(CuLibraryInstruction::ConversionsUsizeToU64Unwrap),
            168 => Ok(CuLibraryInstruction::ConversionsUsizeToU64MapErr),
            170 => Ok(CuLibraryInstruction::ConversionsU64ToUsizeUnwrap),
            171 => Ok(CuLibraryInstruction::ConversionsU64ToUsizeMapErr),
            173 => Ok(CuLibraryInstruction::ConversionsU32ToUsizeUnwrap),
            174 => Ok(CuLibraryInstruction::ConversionsU32ToUsizeMapErr),
            176 => Ok(CuLibraryInstruction::ConversionsU16ToUsizeUnwrap),
            177 => Ok(CuLibraryInstruction::ConversionsU16ToUsizeMapErr),
            179 => Ok(CuLibraryInstruction::ConversionsU8ToUsizeUnwrap),
            180 => Ok(CuLibraryInstruction::ConversionsU8ToUsizeMapErr),
            181 => Ok(CuLibraryInstruction::ConversionsU8AsU16),
            182 => Ok(CuLibraryInstruction::ConversionsU8AsU32),
            183 => Ok(CuLibraryInstruction::ConversionsU8AsU64),
            184 => Ok(CuLibraryInstruction::ConversionsU8AsUsize),
            185 => Ok(CuLibraryInstruction::ConversionsU16AsU8),
            186 => Ok(CuLibraryInstruction::ConversionsU16AsU32),
            187 => Ok(CuLibraryInstruction::ConversionsU16AsU64),
            188 => Ok(CuLibraryInstruction::ConversionsU16AsUsize),
            189 => Ok(CuLibraryInstruction::ConversionsU32AsU8),
            190 => Ok(CuLibraryInstruction::ConversionsU32AsU16),
            191 => Ok(CuLibraryInstruction::ConversionsU32AsU64),
            192 => Ok(CuLibraryInstruction::ConversionsU32AsUsize),
            193 => Ok(CuLibraryInstruction::ConversionsU64AsU8),
            194 => Ok(CuLibraryInstruction::ConversionsU64AsU16),
            195 => Ok(CuLibraryInstruction::ConversionsU64AsU32),
            196 => Ok(CuLibraryInstruction::ConversionsU64AsUsize),
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
    accounts: &[AccountInfo],
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
        CuLibraryInstruction::AccountInfoKey => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let key = account_info_key(&accounts[0]);
            solana_msg::msg!("account key: {:?}", key);
        }
        CuLibraryInstruction::AccountInfoOwner => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let owner = account_info_owner(&accounts[0]);
            solana_msg::msg!("account owner: {:?}", owner);
        }
        CuLibraryInstruction::AccountInfoIsSigner => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_signer = account_info_is_signer(&accounts[0]);
            solana_msg::msg!("is_signer: {}", is_signer);
        }
        CuLibraryInstruction::AccountInfoIsWritable => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_writable = account_info_is_writable(&accounts[0]);
            solana_msg::msg!("is_writable: {}", is_writable);
        }
        CuLibraryInstruction::AccountInfoExecutable => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let executable = account_info_executable(&accounts[0]);
            solana_msg::msg!("executable: {}", executable);
        }
        CuLibraryInstruction::AccountInfoDataLen => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let data_len = account_info_data_len(&accounts[0]);
            solana_msg::msg!("data_len: {}", data_len);
        }
        CuLibraryInstruction::AccountInfoLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let lamports = account_info_lamports(&accounts[0]);
            solana_msg::msg!("lamports: {}", lamports);
        }
        CuLibraryInstruction::AccountInfoDataIsEmpty => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_empty = account_info_data_is_empty(&accounts[0]);
            solana_msg::msg!("data_is_empty: {}", is_empty);
        }
        CuLibraryInstruction::AccountInfoIsOwnedBy => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_owned = account_info_is_owned_by(&accounts[0], program_id);
            solana_msg::msg!("is_owned_by: {}", is_owned);
        }
        CuLibraryInstruction::AccountInfoAssign => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            account_info_assign(&accounts[0], program_id);
            solana_msg::msg!("assigned");
        }
        CuLibraryInstruction::AccountInfoIsBorrowed => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_borrowed = account_info_is_borrowed(&accounts[0]);
            solana_msg::msg!("is_borrowed: {}", is_borrowed);
        }
        CuLibraryInstruction::AccountInfoBorrowLamportsUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let lamports = account_info_borrow_lamports_unchecked(&accounts[0]);
            solana_msg::msg!("lamports: {}", lamports);
        }
        CuLibraryInstruction::AccountInfoBorrowMutLamportsUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let lamports = account_info_borrow_mut_lamports_unchecked(&accounts[0]);
            solana_msg::msg!("lamports: {}", lamports);
        }
        CuLibraryInstruction::AccountInfoBorrowDataUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let data = account_info_borrow_data_unchecked(&accounts[0]);
            solana_msg::msg!("data len: {}", data.len());
        }
        CuLibraryInstruction::AccountInfoBorrowMutDataUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let data = account_info_borrow_mut_data_unchecked(&accounts[0]);
            solana_msg::msg!("data len: {}", data.len());
        }
        CuLibraryInstruction::AccountInfoTryBorrowLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_try_borrow_lamports(&accounts[0])?;
            solana_msg::msg!("borrowed lamports");
        }
        CuLibraryInstruction::AccountInfoTryBorrowMutLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_try_borrow_mut_lamports(&accounts[0])?;
            solana_msg::msg!("borrowed mut lamports");
        }
        CuLibraryInstruction::AccountInfoCanBorrowLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_can_borrow_lamports(&accounts[0])?;
            solana_msg::msg!("can borrow lamports");
        }
        CuLibraryInstruction::AccountInfoCanBorrowMutLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_can_borrow_mut_lamports(&accounts[0])?;
            solana_msg::msg!("can borrow mut lamports");
        }
        CuLibraryInstruction::AccountInfoTryBorrowData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_try_borrow_data(&accounts[0])?;
            solana_msg::msg!("borrowed data");
        }
        CuLibraryInstruction::AccountInfoTryBorrowMutData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_try_borrow_mut_data(&accounts[0])?;
            solana_msg::msg!("borrowed mut data");
        }
        CuLibraryInstruction::AccountInfoCanBorrowData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_can_borrow_data(&accounts[0])?;
            solana_msg::msg!("can borrow data");
        }
        CuLibraryInstruction::AccountInfoCanBorrowMutData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_can_borrow_mut_data(&accounts[0])?;
            solana_msg::msg!("can borrow mut data");
        }
        CuLibraryInstruction::AccountInfoRealloc => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = account_info_realloc(&accounts[0], 1024)?; // Keep same size
            solana_msg::msg!("reallocated");
        }
        CuLibraryInstruction::AccountInfoClose => {
            if accounts.len() < 2 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            // Transfer lamports to payer before closing to avoid unbalanced instruction
            let account = &accounts[0];
            let payer = &accounts[1];
            let lamports = account.lamports();
            if lamports > 0 {
                let mut account_lamports = account.try_borrow_mut_lamports()?;
                let mut payer_lamports = payer.try_borrow_mut_lamports()?;
                *account_lamports -= lamports;
                *payer_lamports += lamports;
            }
            let _ = account_info_close(account)?;
            solana_msg::msg!("closed");
        }
        CuLibraryInstruction::AccountInfoCloseUnchecked => {
            if accounts.len() < 2 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            // Transfer lamports to payer before closing to avoid unbalanced instruction
            let account = &accounts[0];
            let payer = &accounts[1];
            let lamports = account.lamports();
            if lamports > 0 {
                unsafe {
                    *account.borrow_mut_lamports_unchecked() -= lamports;
                    *payer.borrow_mut_lamports_unchecked() += lamports;
                }
            }
            account_info_close_unchecked(account);
            solana_msg::msg!("closed unchecked");
        }
        CuLibraryInstruction::CpiAccountMetaArray10 => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let metas = cpi_account_meta_array_10(&accounts[0..10]);
            solana_msg::msg!("created {} account metas", metas.len());
        }
        CuLibraryInstruction::CpiAccountInfoArray10Ref => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let refs = cpi_account_info_array_10_ref(&accounts[0..10]);
            solana_msg::msg!("created {} account info refs", refs.len());
        }
        CuLibraryInstruction::CpiAccountInfoArray10Clone => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let clones = cpi_account_info_array_10_clone(&accounts[0..10]);
            solana_msg::msg!("cloned {} account infos", clones.len());
        }
        CuLibraryInstruction::CpiAccountInfoArray10Move => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let moved = cpi_account_info_array_10_move(&accounts[0..10]);
            solana_msg::msg!("moved {} account infos", moved.len());
        }
        CuLibraryInstruction::CpiArrayvecPushAccountMeta10 => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = cpi_arrayvec_push_account_meta_10(&accounts[0..10]);
            // Print first account meta to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first meta: {:?}", first.pubkey);
            }
        }
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Ref => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = cpi_arrayvec_push_account_info_10_ref(&accounts[0..10]);
            // Print first account ref to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first ref: {:?}", first.key());
            }
        }
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Clone => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = cpi_arrayvec_push_account_info_10_clone(&accounts[0..10]);
            // Print first cloned account to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first clone: {:?}", first.key());
            }
        }
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Move => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = cpi_arrayvec_push_account_info_10_move(&accounts[0..10]);
            // Print first moved account to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first move: {:?}", first.key());
            }
        }
        CuLibraryInstruction::CpiAccountMetaArray10Loop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let metas = cpi_account_meta_array_10_loop(&accounts[0..10]);
            solana_msg::msg!("first loop meta: {:?}", metas[0].pubkey);
        }
        CuLibraryInstruction::CpiAccountInfoArray10RefLoop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let refs = cpi_account_info_array_10_ref_loop(&accounts[0..10]);
            solana_msg::msg!("first loop ref: {:?}", refs[0].key());
        }
        CuLibraryInstruction::CpiAccountInfoArray10CloneLoop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let clones = cpi_account_info_array_10_clone_loop(&accounts[0..10]);
            solana_msg::msg!("first loop clone: {:?}", clones[0].key());
        }
        CuLibraryInstruction::CpiAccountInfoArray10MoveLoop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let moved = cpi_account_info_array_10_move_loop(&accounts[0..10]);
            solana_msg::msg!("first loop move: {:?}", moved[0].key());
        }
        CuLibraryInstruction::PartialEqU8 => {
            let val = program_id[0];
            let result = partial_eq_u8(val, val);
            solana_msg::msg!("u8 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU16 => {
            let val = u16::from_le_bytes([program_id[0], program_id[1]]);
            let result = partial_eq_u16(val, val);
            solana_msg::msg!("u16 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU32 => {
            let val =
                u32::from_le_bytes([program_id[0], program_id[1], program_id[2], program_id[3]]);
            let result = partial_eq_u32(val, val);
            solana_msg::msg!("u32 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU64 => {
            let val = u64::from_le_bytes([
                program_id[0],
                program_id[1],
                program_id[2],
                program_id[3],
                program_id[4],
                program_id[5],
                program_id[6],
                program_id[7],
            ]);
            let result = partial_eq_u64(val, val);
            solana_msg::msg!("u64 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU128 => {
            let val = u128::from_le_bytes([
                program_id[0],
                program_id[1],
                program_id[2],
                program_id[3],
                program_id[4],
                program_id[5],
                program_id[6],
                program_id[7],
                program_id[8],
                program_id[9],
                program_id[10],
                program_id[11],
                program_id[12],
                program_id[13],
                program_id[14],
                program_id[15],
            ]);
            let result = partial_eq_u128(val, val);
            solana_msg::msg!("u128 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32Ref => {
            // Reference version - just pass references
            let a: &[u8; 32] = program_id;
            let b: &[u8; 32] = program_id;
            let result = partial_eq_array_u8_32_ref(a, b);
            solana_msg::msg!("array u8[32] ref eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32 => {
            // Value version - copy the arrays
            let a: [u8; 32] = *program_id;
            let b: [u8; 32] = *program_id;
            let result = partial_eq_array_u8_32(a, b);
            solana_msg::msg!("array u8[32] eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU16_32 => {
            // Create arrays outside of profiled function
            let val = u16::from_le_bytes([program_id[0], program_id[1]]);
            let a: [u16; 32] = [val; 32];
            let b: [u16; 32] = [val; 32];
            let result = partial_eq_array_u16_32(&a, &b);
            solana_msg::msg!("array u16[32] eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU32_32 => {
            // Create arrays outside of profiled function
            let val =
                u32::from_le_bytes([program_id[0], program_id[1], program_id[2], program_id[3]]);
            let a: [u32; 32] = [val; 32];
            let b: [u32; 32] = [val; 32];
            let result = partial_eq_array_u32_32(&a, &b);
            solana_msg::msg!("array u32[32] eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU64_32 => {
            // Create arrays outside of profiled function
            let val = u64::from_le_bytes([
                program_id[0],
                program_id[1],
                program_id[2],
                program_id[3],
                program_id[4],
                program_id[5],
                program_id[6],
                program_id[7],
            ]);
            let a: [u64; 32] = [val; 32];
            let b: [u64; 32] = [val; 32];
            let result = partial_eq_array_u64_32(&a, &b);
            solana_msg::msg!("array u64[32] eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU8Neq => {
            let val1 = program_id[0];
            let val2 = program_id[1];
            let result = partial_eq_u8_neq(val1, val2);
            solana_msg::msg!("u8 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqU16Neq => {
            let val1 = u16::from_le_bytes([program_id[0], program_id[1]]);
            let val2 = u16::from_le_bytes([program_id[2], program_id[3]]);
            let result = partial_eq_u16_neq(val1, val2);
            solana_msg::msg!("u16 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqU32Neq => {
            let val1 =
                u32::from_le_bytes([program_id[0], program_id[1], program_id[2], program_id[3]]);
            let val2 =
                u32::from_le_bytes([program_id[4], program_id[5], program_id[6], program_id[7]]);
            let result = partial_eq_u32_neq(val1, val2);
            solana_msg::msg!("u32 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqU64Neq => {
            let val1 = u64::from_le_bytes([
                program_id[0],
                program_id[1],
                program_id[2],
                program_id[3],
                program_id[4],
                program_id[5],
                program_id[6],
                program_id[7],
            ]);
            let val2 = u64::from_le_bytes([
                program_id[8],
                program_id[9],
                program_id[10],
                program_id[11],
                program_id[12],
                program_id[13],
                program_id[14],
                program_id[15],
            ]);
            let result = partial_eq_u64_neq(val1, val2);
            solana_msg::msg!("u64 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqU128Neq => {
            let val1 = u128::from_le_bytes([
                program_id[0],
                program_id[1],
                program_id[2],
                program_id[3],
                program_id[4],
                program_id[5],
                program_id[6],
                program_id[7],
                program_id[8],
                program_id[9],
                program_id[10],
                program_id[11],
                program_id[12],
                program_id[13],
                program_id[14],
                program_id[15],
            ]);
            let val2 = u128::from_le_bytes([
                program_id[16],
                program_id[17],
                program_id[18],
                program_id[19],
                program_id[20],
                program_id[21],
                program_id[22],
                program_id[23],
                program_id[24],
                program_id[25],
                program_id[26],
                program_id[27],
                program_id[28],
                program_id[29],
                program_id[30],
                program_id[31],
            ]);
            let result = partial_eq_u128_neq(val1, val2);
            solana_msg::msg!("u128 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32NeqRef => {
            // Reference version - create arrays with different last element outside profiled function
            let val1 = program_id[0];
            let val2 = program_id[1];
            let a: [u8; 32] = [val1; 32];
            let mut b: [u8; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = partial_eq_array_u8_32_neq_ref(&a, &b);
            solana_msg::msg!("array u8[32] neq ref: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32Neq => {
            // Value version - create arrays with different last element outside profiled function
            let val1 = program_id[0];
            let val2 = program_id[1];
            let a: [u8; 32] = [val1; 32];
            let mut b: [u8; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = partial_eq_array_u8_32_neq(a, b);
            solana_msg::msg!("array u8[32] neq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32NeqDeref => {
            // Dereference version - dereferences inside profiled function
            let val1 = program_id[0];
            let val2 = program_id[1];
            let a: [u8; 32] = [val1; 32];
            let mut b: [u8; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = partial_eq_array_u8_32_neq_deref(&a, &b);
            solana_msg::msg!("array u8[32] neq deref: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU16_32Neq => {
            // Create arrays with different last element outside profiled function
            let val1 = u16::from_le_bytes([program_id[0], program_id[1]]);
            let val2 = u16::from_le_bytes([program_id[2], program_id[3]]);
            let a: [u16; 32] = [val1; 32];
            let mut b: [u16; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = partial_eq_array_u16_32_neq(&a, &b);
            solana_msg::msg!("array u16[32] neq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU32_32Neq => {
            // Create arrays with different last element outside profiled function
            let val1 =
                u32::from_le_bytes([program_id[0], program_id[1], program_id[2], program_id[3]]);
            let val2 =
                u32::from_le_bytes([program_id[4], program_id[5], program_id[6], program_id[7]]);
            let a: [u32; 32] = [val1; 32];
            let mut b: [u32; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = partial_eq_array_u32_32_neq(&a, &b);
            solana_msg::msg!("array u32[32] neq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU64_32Neq => {
            // Create arrays with different last element outside profiled function
            let val1 = u64::from_le_bytes([
                program_id[0],
                program_id[1],
                program_id[2],
                program_id[3],
                program_id[4],
                program_id[5],
                program_id[6],
                program_id[7],
            ]);
            let val2 = u64::from_le_bytes([
                program_id[8],
                program_id[9],
                program_id[10],
                program_id[11],
                program_id[12],
                program_id[13],
                program_id[14],
                program_id[15],
            ]);
            let a: [u64; 32] = [val1; 32];
            let mut b: [u64; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = partial_eq_array_u64_32_neq(&a, &b);
            solana_msg::msg!("array u64[32] neq: {}", result);
        }
        // Conversion benchmarks
        CuLibraryInstruction::ConversionsSliceToArray32Unwrap => {
            let slice = &program_id[..32];
            let arr = conversions_try_into_slice_to_array_32_unwrap(slice);
            solana_msg::msg!("slice to array unwrap: {:?}", arr[0]);
        }
        CuLibraryInstruction::ConversionsSliceToArray32MapErr => {
            let slice = &program_id[..32];
            let result = conversions_try_into_slice_to_array_32_map_err(slice);
            solana_msg::msg!("slice to array map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsUsizeToU64Unwrap => {
            let val: usize = 42;
            let result = conversions_try_into_usize_to_u64_unwrap(val);
            solana_msg::msg!("usize to u64 unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsUsizeToU64MapErr => {
            let val: usize = 42;
            let result = conversions_try_into_usize_to_u64_map_err(val);
            solana_msg::msg!("usize to u64 map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU64ToUsizeUnwrap => {
            let val: u64 = 42;
            let result = conversions_try_into_u64_to_usize_unwrap(val);
            solana_msg::msg!("u64 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU64ToUsizeMapErr => {
            let val: u64 = 42;
            let result = conversions_try_into_u64_to_usize_map_err(val);
            solana_msg::msg!("u64 to usize map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU32ToUsizeUnwrap => {
            let val: u32 = 42;
            let result = conversions_try_into_u32_to_usize_unwrap(val);
            solana_msg::msg!("u32 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU32ToUsizeMapErr => {
            let val: u32 = 42;
            let result = conversions_try_into_u32_to_usize_map_err(val);
            solana_msg::msg!("u32 to usize map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU16ToUsizeUnwrap => {
            let val: u16 = 42;
            let result = conversions_try_into_u16_to_usize_unwrap(val);
            solana_msg::msg!("u16 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU16ToUsizeMapErr => {
            let val: u16 = 42;
            let result = conversions_try_into_u16_to_usize_map_err(val);
            solana_msg::msg!("u16 to usize map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU8ToUsizeUnwrap => {
            let val: u8 = 42;
            let result = conversions_try_into_u8_to_usize_unwrap(val);
            solana_msg::msg!("u8 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU8ToUsizeMapErr => {
            let val: u8 = 42;
            let result = conversions_try_into_u8_to_usize_map_err(val);
            solana_msg::msg!("u8 to usize map_err: {:?}", result.is_ok());
        }
        // Cast conversions
        CuLibraryInstruction::ConversionsU8AsU16 => {
            let val: u8 = 42;
            let result = conversions_u8_as_u16(val);
            solana_msg::msg!("u8 as u16: {}", result);
        }
        CuLibraryInstruction::ConversionsU8AsU32 => {
            let val: u8 = 42;
            let result = conversions_u8_as_u32(val);
            solana_msg::msg!("u8 as u32: {}", result);
        }
        CuLibraryInstruction::ConversionsU8AsU64 => {
            let val: u8 = 42;
            let result = conversions_u8_as_u64(val);
            solana_msg::msg!("u8 as u64: {}", result);
        }
        CuLibraryInstruction::ConversionsU8AsUsize => {
            let val: u8 = 42;
            let result = conversions_u8_as_usize(val);
            solana_msg::msg!("u8 as usize: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsU8 => {
            let val: u16 = 300;
            let result = conversions_u16_as_u8(val);
            solana_msg::msg!("u16 as u8: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsU32 => {
            let val: u16 = 300;
            let result = conversions_u16_as_u32(val);
            solana_msg::msg!("u16 as u32: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsU64 => {
            let val: u16 = 300;
            let result = conversions_u16_as_u64(val);
            solana_msg::msg!("u16 as u64: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsUsize => {
            let val: u16 = 300;
            let result = conversions_u16_as_usize(val);
            solana_msg::msg!("u16 as usize: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsU8 => {
            let val: u32 = 70000;
            let result = conversions_u32_as_u8(val);
            solana_msg::msg!("u32 as u8: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsU16 => {
            let val: u32 = 70000;
            let result = conversions_u32_as_u16(val);
            solana_msg::msg!("u32 as u16: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsU64 => {
            let val: u32 = 70000;
            let result = conversions_u32_as_u64(val);
            solana_msg::msg!("u32 as u64: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsUsize => {
            let val: u32 = 70000;
            let result = conversions_u32_as_usize(val);
            solana_msg::msg!("u32 as usize: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsU8 => {
            let val: u64 = 70000;
            let result = conversions_u64_as_u8(val);
            solana_msg::msg!("u64 as u8: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsU16 => {
            let val: u64 = 70000;
            let result = conversions_u64_as_u16(val);
            solana_msg::msg!("u64 as u16: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsU32 => {
            let val: u64 = 70000;
            let result = conversions_u64_as_u32(val);
            solana_msg::msg!("u64 as u32: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsUsize => {
            let val: u64 = 70000;
            let result = conversions_u64_as_usize(val);
            solana_msg::msg!("u64 as usize: {}", result);
        }
    }
    Ok(())
}
