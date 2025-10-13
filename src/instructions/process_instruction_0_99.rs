use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{baseline_empty_function, instructions::discriminator::CuLibraryInstruction};
use crate::{collections, math, pinocchio_ops, solana_ops};

use crate::collections::array::array_assign::{
    assign_10_pubkey, assign_10_u64, assign_10_u8, assign_pubkey, assign_u64, assign_u8,
};
use crate::collections::array::array_new::new;

use crate::collections::array::array_with_capacity::{with_capacity_10, with_capacity_100};

use crate::math::std_math::add_assign::{
    add_assign_u128, add_assign_u16, add_assign_u32, add_assign_u64, add_assign_u8,
};
use crate::math::std_math::sub_assign::{
    sub_assign_u128, sub_assign_u16, sub_assign_u32, sub_assign_u64, sub_assign_u8,
};
#[inline(never)]
pub fn process_instruction_0_99(
    instruction: CuLibraryInstruction,
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    match instruction {
        CuLibraryInstruction::Baseline => {
            baseline_empty_function();
            solana_msg::msg!("baseline complete");
        }
        CuLibraryInstruction::Msg10 => pinocchio_ops::msg::msg10_chars()?,
        CuLibraryInstruction::SolanaMsg10 => solana_ops::msg::msg10_chars()?,
        CuLibraryInstruction::SolanaMsgProgramId => {
            solana_ops::msg_program_id::msg_program_id(program_id)?
        }
        CuLibraryInstruction::SolanaPubkeyNewFromArray => {
            let res = solana_ops::pubkey_new_from_array::pubkey_new_from_array(program_id);
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::PinocchioSysvarRentExemption165 => {
            let _ = pinocchio_ops::sysvar_rent::sysvar_rent_exemption_165();
        }
        CuLibraryInstruction::PinocchioClockGetSlot => {
            pinocchio_ops::sysvar_clock::clock_get_slot()?
        }
        CuLibraryInstruction::ArrayvecNew => {
            let res = collections::arrayvec::vec_new::u8_new();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecWithCapacity10 => {
            let res = collections::arrayvec::vec_with_capacity::u8_with_capacity_10();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecWithCapacity100 => {
            let res = collections::arrayvec::vec_with_capacity::u8_with_capacity_100();
            solana_msg::msg!("vec: {:?}", res.as_slice());
        }
        CuLibraryInstruction::ArrayvecPushU8 => {
            let mut vec = collections::arrayvec::vec_push::create_empty_u8_vec();
            collections::arrayvec::vec_push::push_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::ArrayvecPushU64 => {
            let mut vec = collections::arrayvec::vec_push::create_empty_u64_vec();
            collections::arrayvec::vec_push::push_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::ArrayvecPushPubkey => {
            let mut vec = collections::arrayvec::vec_push::create_empty_pubkey_vec();
            collections::arrayvec::vec_push::push_pubkey(&mut vec, program_id);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::ArrayvecPush10U8 => {
            let mut vec = collections::arrayvec::vec_push::create_empty_u8_vec();
            collections::arrayvec::vec_push::push_10_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::ArrayvecPush10U64 => {
            let mut vec = collections::arrayvec::vec_push::create_empty_u64_vec();
            collections::arrayvec::vec_push::push_10_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::ArrayvecPush10Pubkey => {
            let mut vec = collections::arrayvec::vec_push::create_empty_pubkey_vec();
            collections::arrayvec::vec_push::push_10_pubkey(&mut vec, program_id)?;
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::VecNew => {
            let res = collections::vec::vec_new::u8_new();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecWithCapacity10 => {
            let res = collections::vec::vec_with_capacity::u8_with_capacity_10();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecWithCapacity100 => {
            let res = collections::vec::vec_with_capacity::u8_with_capacity_100();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushU8 => {
            let res = collections::vec::vec_push::push_u8();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushU64 => {
            let res = collections::vec::vec_push::push_u64();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushPubkey => {
            let res = collections::vec::vec_push::push_pubkey(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U8 => {
            let res = collections::vec::vec_push::push_10_u8();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U64 => {
            let res = collections::vec::vec_push::push_10_u64();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10Pubkey => {
            let res = collections::vec::vec_push::push_10_pubkey(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::ArrayNew => {
            let res = new();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayWithCapacity10 => {
            let res = with_capacity_10();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayWithCapacity100 => {
            let res = with_capacity_100();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssignU8 => {
            let res = assign_u8();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssignU64 => {
            let res = assign_u64();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssignPubkey => {
            let res = assign_pubkey(program_id);
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssign10U8 => {
            let res = assign_10_u8();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssign10U64 => {
            let res = assign_10_u64();
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::ArrayAssign10Pubkey => {
            let res = assign_10_pubkey(program_id);
            solana_msg::msg!("array: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU8 => {
            let res = math::checked_math::checked_add::add_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU16 => {
            let res = math::checked_math::checked_add::add_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU32 => {
            let res = math::checked_math::checked_add::add_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU64 => {
            let res = math::checked_math::checked_add::add_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedAddU128 => {
            let res = math::checked_math::checked_add::add_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU8 => {
            let res = math::checked_math::checked_sub::sub_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU16 => {
            let res = math::checked_math::checked_sub::sub_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU32 => {
            let res = math::checked_math::checked_sub::sub_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU64 => {
            let res = math::checked_math::checked_sub::sub_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedSubU128 => {
            let res = math::checked_math::checked_sub::sub_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU8 => {
            let res = math::checked_math::checked_mul::mul_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU16 => {
            let res = math::checked_math::checked_mul::mul_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU32 => {
            let res = math::checked_math::checked_mul::mul_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU64 => {
            let res = math::checked_math::checked_mul::mul_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedMulU128 => {
            let res = math::checked_math::checked_mul::mul_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU8 => {
            let res = math::checked_math::checked_div::div_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU16 => {
            let res = math::checked_math::checked_div::div_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU32 => {
            let res = math::checked_math::checked_div::div_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU64 => {
            let res = math::checked_math::checked_div::div_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::CheckedDivU128 => {
            let res = math::checked_math::checked_div::div_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU8 => {
            let res = math::saturating_math::saturating_add::add_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU16 => {
            let res = math::saturating_math::saturating_add::add_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU32 => {
            let res = math::saturating_math::saturating_add::add_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU64 => {
            let res = math::saturating_math::saturating_add::add_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingAddU128 => {
            let res = math::saturating_math::saturating_add::add_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU8 => {
            let res = math::saturating_math::saturating_sub::sub_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU16 => {
            let res = math::saturating_math::saturating_sub::sub_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU32 => {
            let res = math::saturating_math::saturating_sub::sub_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU64 => {
            let res = math::saturating_math::saturating_sub::sub_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingSubU128 => {
            let res = math::saturating_math::saturating_sub::sub_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU8 => {
            let res = math::saturating_math::saturating_mul::mul_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU16 => {
            let res = math::saturating_math::saturating_mul::mul_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU32 => {
            let res = math::saturating_math::saturating_mul::mul_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU64 => {
            let res = math::saturating_math::saturating_mul::mul_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::SaturatingMulU128 => {
            let res = math::saturating_math::saturating_mul::mul_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU8 => {
            let res = math::std_math::std_add::add_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU16 => {
            let res = math::std_math::std_add::add_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU32 => {
            let res = math::std_math::std_add::add_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU64 => {
            let res = math::std_math::std_add::add_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdAddU128 => {
            let res = math::std_math::std_add::add_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU8 => {
            let res = math::std_math::std_sub::sub_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU16 => {
            let res = math::std_math::std_sub::sub_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU32 => {
            let res = math::std_math::std_sub::sub_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU64 => {
            let res = math::std_math::std_sub::sub_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdSubU128 => {
            let res = math::std_math::std_sub::sub_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU8 => {
            let res = math::std_math::std_mul::mul_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU16 => {
            let res = math::std_math::std_mul::mul_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU32 => {
            let res = math::std_math::std_mul::mul_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU64 => {
            let res = math::std_math::std_mul::mul_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdMulU128 => {
            let res = math::std_math::std_mul::mul_u128();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU8 => {
            let res = math::std_math::std_div::div_u8();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU16 => {
            let res = math::std_math::std_div::div_u16();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU32 => {
            let res = math::std_math::std_div::div_u32();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU64 => {
            let res = math::std_math::std_div::div_u64();
            solana_msg::msg!("result: {:?}", res);
        }
        CuLibraryInstruction::StdDivU128 => {
            let res = math::std_math::std_div::div_u128();
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
            let res = collections::vec::vec_push::push_u8_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
