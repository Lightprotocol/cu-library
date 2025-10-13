use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{collections, instructions::discriminator::CuLibraryInstruction};

use crate::account_info::account_borrows::{
    borrow_data_unchecked, borrow_lamports_unchecked, borrow_mut_data_unchecked,
    borrow_mut_lamports_unchecked, can_borrow_data, can_borrow_lamports, can_borrow_mut_data,
    can_borrow_mut_lamports, is_borrowed, try_borrow_data, try_borrow_lamports,
    try_borrow_mut_data, try_borrow_mut_lamports,
};
use crate::account_info::account_checks::{data_is_empty, executable, is_signer, is_writable};
use crate::account_info::account_data::{data_len, lamports};
use crate::account_info::account_key::key;
use crate::account_info::account_owner::owner;
use crate::account_info::account_ownership::{assign, is_owned_by};
use crate::account_info::account_realloc::{close, close_unchecked, realloc};

use crate::conversions::cast_u16::{u16_as_u32, u16_as_u64, u16_as_u8, u16_as_usize};
use crate::conversions::cast_u32::{u32_as_u16, u32_as_u64, u32_as_u8, u32_as_usize};
use crate::conversions::cast_u64::{u64_as_u16, u64_as_u32, u64_as_u8, u64_as_usize};
use crate::conversions::cast_u8::{u8_as_u16, u8_as_u32, u8_as_u64, u8_as_usize};
use crate::conversions::from_usize::{try_into_usize_to_u64_map_err, try_into_usize_to_u64_unwrap};
use crate::conversions::slice_to_array::{
    try_into_slice_to_array_32_map_err, try_into_slice_to_array_32_unwrap,
};
use crate::conversions::to_usize::{
    try_into_u16_to_usize_map_err, try_into_u16_to_usize_unwrap, try_into_u32_to_usize_map_err,
    try_into_u32_to_usize_unwrap, try_into_u64_to_usize_map_err, try_into_u64_to_usize_unwrap,
    try_into_u8_to_usize_map_err, try_into_u8_to_usize_unwrap,
};
use crate::cpi::cpi_array_loop::{
    account_info_array_10_clone_loop, account_info_array_10_move_loop,
    account_info_array_10_ref_loop, account_meta_array_10_loop,
};
use crate::cpi::cpi_arrays::{
    account_info_array_10_clone, account_info_array_10_move, account_info_array_10_ref,
    account_meta_array_10,
};
use crate::cpi::cpi_arrayvec::{
    arrayvec_push_account_info_10_clone, arrayvec_push_account_info_10_move,
    arrayvec_push_account_info_10_ref, arrayvec_push_account_meta_10,
};
use crate::option::option_checked_add::{
    checked_add_u8_ok_or, checked_add_u8_ok_or_else, checked_add_u8_unwrap,
};

use crate::partial_eq::partial_eq_arrays::{
    array_u16_32, array_u32_32, array_u64_32, array_u8_32, array_u8_32_ref,
};
use crate::partial_eq::partial_eq_neq::{
    array_u16_32_neq, array_u32_32_neq, array_u64_32_neq, array_u8_32_neq, array_u8_32_neq_deref,
    array_u8_32_neq_ref, u128_neq, u16_neq, u32_neq, u64_neq, u8_neq,
};
use crate::partial_eq::partial_eq_primitives::{u128, u16, u32, u64, u8};

#[inline(never)]
pub fn process_instruction_100_199(
    instruction: CuLibraryInstruction,
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    match instruction {
        CuLibraryInstruction::VecPushU64WithCapacity => {
            let res = collections::vec::vec_push::push_u64_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPushPubkeyWithCapacity => {
            let res = collections::vec::vec_push::push_pubkey_with_capacity(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U8WithCapacity => {
            let res = collections::vec::vec_push::push_10_u8_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10U64WithCapacity => {
            let res = collections::vec::vec_push::push_10_u64_with_capacity();
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::VecPush10PubkeyWithCapacity => {
            let res = collections::vec::vec_push::push_10_pubkey_with_capacity(program_id);
            solana_msg::msg!("vec: {:?}", res);
        }
        CuLibraryInstruction::AccountInfoKey => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let key = key(&accounts[0]);
            solana_msg::msg!("account key: {:?}", key);
        }
        CuLibraryInstruction::AccountInfoOwner => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let owner = owner(&accounts[0]);
            solana_msg::msg!("account owner: {:?}", owner);
        }
        CuLibraryInstruction::AccountInfoIsSigner => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_signer = is_signer(&accounts[0]);
            solana_msg::msg!("is_signer: {}", is_signer);
        }
        CuLibraryInstruction::AccountInfoIsWritable => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_writable = is_writable(&accounts[0]);
            solana_msg::msg!("is_writable: {}", is_writable);
        }
        CuLibraryInstruction::AccountInfoExecutable => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let executable = executable(&accounts[0]);
            solana_msg::msg!("executable: {}", executable);
        }
        CuLibraryInstruction::AccountInfoDataLen => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let data_len = data_len(&accounts[0]);
            solana_msg::msg!("data_len: {}", data_len);
        }
        CuLibraryInstruction::AccountInfoLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let lamports = lamports(&accounts[0]);
            solana_msg::msg!("lamports: {}", lamports);
        }
        CuLibraryInstruction::AccountInfoDataIsEmpty => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_empty = data_is_empty(&accounts[0]);
            solana_msg::msg!("data_is_empty: {}", is_empty);
        }
        CuLibraryInstruction::AccountInfoIsOwnedBy => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_owned = is_owned_by(&accounts[0], program_id);
            solana_msg::msg!("is_owned_by: {}", is_owned);
        }
        CuLibraryInstruction::AccountInfoAssign => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            assign(&accounts[0], program_id);
            solana_msg::msg!("assigned");
        }
        CuLibraryInstruction::AccountInfoIsBorrowed => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let is_borrowed = is_borrowed(&accounts[0]);
            solana_msg::msg!("is_borrowed: {}", is_borrowed);
        }
        CuLibraryInstruction::AccountInfoBorrowLamportsUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let lamports = borrow_lamports_unchecked(&accounts[0]);
            solana_msg::msg!("lamports: {}", lamports);
        }
        CuLibraryInstruction::AccountInfoBorrowMutLamportsUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let lamports = borrow_mut_lamports_unchecked(&accounts[0]);
            solana_msg::msg!("lamports: {}", lamports);
        }
        CuLibraryInstruction::AccountInfoBorrowDataUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let data = borrow_data_unchecked(&accounts[0]);
            solana_msg::msg!("data len: {}", data.len());
        }
        CuLibraryInstruction::AccountInfoBorrowMutDataUnchecked => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let data = borrow_mut_data_unchecked(&accounts[0]);
            solana_msg::msg!("data len: {}", data.len());
        }
        CuLibraryInstruction::AccountInfoTryBorrowLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = try_borrow_lamports(&accounts[0])?;
            solana_msg::msg!("borrowed lamports");
        }
        CuLibraryInstruction::AccountInfoTryBorrowMutLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = try_borrow_mut_lamports(&accounts[0])?;
            solana_msg::msg!("borrowed mut lamports");
        }
        CuLibraryInstruction::AccountInfoCanBorrowLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = can_borrow_lamports(&accounts[0])?;
            solana_msg::msg!("can borrow lamports");
        }
        CuLibraryInstruction::AccountInfoCanBorrowMutLamports => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = can_borrow_mut_lamports(&accounts[0])?;
            solana_msg::msg!("can borrow mut lamports");
        }
        CuLibraryInstruction::AccountInfoTryBorrowData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = try_borrow_data(&accounts[0])?;
            solana_msg::msg!("borrowed data");
        }
        CuLibraryInstruction::AccountInfoTryBorrowMutData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = try_borrow_mut_data(&accounts[0])?;
            solana_msg::msg!("borrowed mut data");
        }
        CuLibraryInstruction::AccountInfoCanBorrowData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = can_borrow_data(&accounts[0])?;
            solana_msg::msg!("can borrow data");
        }
        CuLibraryInstruction::AccountInfoCanBorrowMutData => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = can_borrow_mut_data(&accounts[0])?;
            solana_msg::msg!("can borrow mut data");
        }
        CuLibraryInstruction::AccountInfoRealloc => {
            if accounts.is_empty() {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let _ = realloc(&accounts[0], 1024)?; // Keep same size
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
                // Update account lamports first, then drop the borrow
                {
                    let mut account_lamports = account.try_borrow_mut_lamports()?;
                    *account_lamports = 0;
                }
                // Now update payer lamports
                {
                    let mut payer_lamports = payer.try_borrow_mut_lamports()?;
                    *payer_lamports += lamports;
                }
            }
            let _ = close(account)?;
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
                    // Update account lamports first
                    *account.borrow_mut_lamports_unchecked() = 0;
                }
                unsafe {
                    // Now update payer lamports
                    *payer.borrow_mut_lamports_unchecked() += lamports;
                }
            }
            close_unchecked(account);
            solana_msg::msg!("closed unchecked");
        }
        CuLibraryInstruction::CpiAccountMetaArray10 => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let metas = account_meta_array_10(&accounts[0..10]);
            solana_msg::msg!("created {} account metas", metas.len());
        }
        CuLibraryInstruction::CpiAccountInfoArray10Ref => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let refs = account_info_array_10_ref(&accounts[0..10]);
            solana_msg::msg!("created {} account info refs", refs.len());
        }
        CuLibraryInstruction::CpiAccountInfoArray10Clone => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let clones = account_info_array_10_clone(&accounts[0..10]);
            solana_msg::msg!("cloned {} account infos", clones.len());
        }
        CuLibraryInstruction::CpiAccountInfoArray10Move => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let moved = account_info_array_10_move(&accounts[0..10]);
            solana_msg::msg!("moved {} account infos", moved.len());
        }
        CuLibraryInstruction::CpiArrayvecPushAccountMeta10 => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = arrayvec_push_account_meta_10(&accounts[0..10]);
            // Print first account meta to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first meta: {:?}", first.pubkey);
            }
        }
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Ref => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = arrayvec_push_account_info_10_ref(&accounts[0..10]);
            // Print first account ref to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first ref: {:?}", first.key());
            }
        }
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Clone => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = arrayvec_push_account_info_10_clone(&accounts[0..10]);
            // Print first cloned account to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first clone: {:?}", first.key());
            }
        }
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Move => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let vec = arrayvec_push_account_info_10_move(&accounts[0..10]);
            // Print first moved account to prevent optimization
            if let Some(first) = vec.first() {
                solana_msg::msg!("first move: {:?}", first.key());
            }
        }
        CuLibraryInstruction::CpiAccountMetaArray10Loop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let metas = account_meta_array_10_loop(&accounts[0..10]);
            solana_msg::msg!("first loop meta: {:?}", metas[0].pubkey);
        }
        CuLibraryInstruction::CpiAccountInfoArray10RefLoop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let refs = account_info_array_10_ref_loop(&accounts[0..10]);
            solana_msg::msg!("first loop ref: {:?}", refs[0].key());
        }
        CuLibraryInstruction::CpiAccountInfoArray10CloneLoop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let clones = account_info_array_10_clone_loop(&accounts[0..10]);
            solana_msg::msg!("first loop clone: {:?}", clones[0].key());
        }
        CuLibraryInstruction::CpiAccountInfoArray10MoveLoop => {
            if accounts.len() < 10 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            let moved = account_info_array_10_move_loop(&accounts[0..10]);
            solana_msg::msg!("first loop move: {:?}", moved[0].key());
        }
        CuLibraryInstruction::PartialEqU8 => {
            let val = program_id[0];
            let result = u8(val, val);
            solana_msg::msg!("u8 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU16 => {
            let val = u16::from_le_bytes([program_id[0], program_id[1]]);
            let result = u16(val, val);
            solana_msg::msg!("u16 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU32 => {
            let val =
                u32::from_le_bytes([program_id[0], program_id[1], program_id[2], program_id[3]]);
            let result = u32(val, val);
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
            let result = u64(val, val);
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
            let result = u128(val, val);
            solana_msg::msg!("u128 eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32Ref => {
            // Reference version - just pass references
            let a: &[u8; 32] = program_id;
            let b: &[u8; 32] = program_id;
            let result = array_u8_32_ref(a, b);
            solana_msg::msg!("array u8[32] ref eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32 => {
            // Value version - copy the arrays
            let a: [u8; 32] = *program_id;
            let b: [u8; 32] = *program_id;
            let result = array_u8_32(a, b);
            solana_msg::msg!("array u8[32] eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU16_32 => {
            // Create arrays outside of profiled function
            let val = u16::from_le_bytes([program_id[0], program_id[1]]);
            let a: [u16; 32] = [val; 32];
            let b: [u16; 32] = [val; 32];
            let result = array_u16_32(&a, &b);
            solana_msg::msg!("array u16[32] eq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU32_32 => {
            // Create arrays outside of profiled function
            let val =
                u32::from_le_bytes([program_id[0], program_id[1], program_id[2], program_id[3]]);
            let a: [u32; 32] = [val; 32];
            let b: [u32; 32] = [val; 32];
            let result = array_u32_32(&a, &b);
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
            let result = array_u64_32(&a, &b);
            solana_msg::msg!("array u64[32] eq: {}", result);
        }
        CuLibraryInstruction::PartialEqU8Neq => {
            let val1 = program_id[0];
            let val2 = program_id[1];
            let result = u8_neq(val1, val2);
            solana_msg::msg!("u8 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqU16Neq => {
            let val1 = u16::from_le_bytes([program_id[0], program_id[1]]);
            let val2 = u16::from_le_bytes([program_id[2], program_id[3]]);
            let result = u16_neq(val1, val2);
            solana_msg::msg!("u16 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqU32Neq => {
            let val1 =
                u32::from_le_bytes([program_id[0], program_id[1], program_id[2], program_id[3]]);
            let val2 =
                u32::from_le_bytes([program_id[4], program_id[5], program_id[6], program_id[7]]);
            let result = u32_neq(val1, val2);
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
            let result = u64_neq(val1, val2);
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
            let result = u128_neq(val1, val2);
            solana_msg::msg!("u128 neq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32NeqRef => {
            // Reference version - create arrays with different last element outside profiled function
            let val1 = program_id[0];
            let val2 = program_id[1];
            let a: [u8; 32] = [val1; 32];
            let mut b: [u8; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = array_u8_32_neq_ref(&a, &b);
            solana_msg::msg!("array u8[32] neq ref: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32Neq => {
            // Value version - create arrays with different last element outside profiled function
            let val1 = program_id[0];
            let val2 = program_id[1];
            let a: [u8; 32] = [val1; 32];
            let mut b: [u8; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = array_u8_32_neq(a, b);
            solana_msg::msg!("array u8[32] neq: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU8_32NeqDeref => {
            // Dereference version - dereferences inside profiled function
            let val1 = program_id[0];
            let val2 = program_id[1];
            let a: [u8; 32] = [val1; 32];
            let mut b: [u8; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = array_u8_32_neq_deref(&a, &b);
            solana_msg::msg!("array u8[32] neq deref: {}", result);
        }
        CuLibraryInstruction::PartialEqArrayU16_32Neq => {
            // Create arrays with different last element outside profiled function
            let val1 = u16::from_le_bytes([program_id[0], program_id[1]]);
            let val2 = u16::from_le_bytes([program_id[2], program_id[3]]);
            let a: [u16; 32] = [val1; 32];
            let mut b: [u16; 32] = [val1; 32];
            b[31] = val2; // Make last element different
            let result = array_u16_32_neq(&a, &b);
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
            let result = array_u32_32_neq(&a, &b);
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
            let result = array_u64_32_neq(&a, &b);
            solana_msg::msg!("array u64[32] neq: {}", result);
        }
        // Conversion benchmarks
        CuLibraryInstruction::ConversionsSliceToArray32Unwrap => {
            let slice = &program_id[..32];
            let arr = try_into_slice_to_array_32_unwrap(slice);
            solana_msg::msg!("slice to array unwrap: {:?}", arr[0]);
        }
        CuLibraryInstruction::ConversionsSliceToArray32MapErr => {
            let slice = &program_id[..32];
            let result = try_into_slice_to_array_32_map_err(slice);
            solana_msg::msg!("slice to array map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsUsizeToU64Unwrap => {
            let val: usize = 42;
            let result = try_into_usize_to_u64_unwrap(val);
            solana_msg::msg!("usize to u64 unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsUsizeToU64MapErr => {
            let val: usize = 42;
            let result = try_into_usize_to_u64_map_err(val);
            solana_msg::msg!("usize to u64 map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU64ToUsizeUnwrap => {
            let val: u64 = 42;
            let result = try_into_u64_to_usize_unwrap(val);
            solana_msg::msg!("u64 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU64ToUsizeMapErr => {
            let val: u64 = 42;
            let result = try_into_u64_to_usize_map_err(val);
            solana_msg::msg!("u64 to usize map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU32ToUsizeUnwrap => {
            let val: u32 = 42;
            let result = try_into_u32_to_usize_unwrap(val);
            solana_msg::msg!("u32 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU32ToUsizeMapErr => {
            let val: u32 = 42;
            let result = try_into_u32_to_usize_map_err(val);
            solana_msg::msg!("u32 to usize map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU16ToUsizeUnwrap => {
            let val: u16 = 42;
            let result = try_into_u16_to_usize_unwrap(val);
            solana_msg::msg!("u16 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU16ToUsizeMapErr => {
            let val: u16 = 42;
            let result = try_into_u16_to_usize_map_err(val);
            solana_msg::msg!("u16 to usize map_err: {:?}", result.is_ok());
        }
        CuLibraryInstruction::ConversionsU8ToUsizeUnwrap => {
            let val: u8 = 42;
            let result = try_into_u8_to_usize_unwrap(val);
            solana_msg::msg!("u8 to usize unwrap: {}", result);
        }
        CuLibraryInstruction::ConversionsU8ToUsizeMapErr => {
            let val: u8 = 42;
            let result = try_into_u8_to_usize_map_err(val);
            solana_msg::msg!("u8 to usize map_err: {:?}", result.is_ok());
        }
        // Cast conversions
        CuLibraryInstruction::ConversionsU8AsU16 => {
            let val: u8 = 42;
            let result = u8_as_u16(val);
            solana_msg::msg!("u8 as u16: {}", result);
        }
        CuLibraryInstruction::ConversionsU8AsU32 => {
            let val: u8 = 42;
            let result = u8_as_u32(val);
            solana_msg::msg!("u8 as u32: {}", result);
        }
        CuLibraryInstruction::ConversionsU8AsU64 => {
            let val: u8 = 42;
            let result = u8_as_u64(val);
            solana_msg::msg!("u8 as u64: {}", result);
        }
        CuLibraryInstruction::ConversionsU8AsUsize => {
            let val: u8 = 42;
            let result = u8_as_usize(val);
            solana_msg::msg!("u8 as usize: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsU8 => {
            let val: u16 = 300;
            let result = u16_as_u8(val);
            solana_msg::msg!("u16 as u8: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsU32 => {
            let val: u16 = 300;
            let result = u16_as_u32(val);
            solana_msg::msg!("u16 as u32: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsU64 => {
            let val: u16 = 300;
            let result = u16_as_u64(val);
            solana_msg::msg!("u16 as u64: {}", result);
        }
        CuLibraryInstruction::ConversionsU16AsUsize => {
            let val: u16 = 300;
            let result = u16_as_usize(val);
            solana_msg::msg!("u16 as usize: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsU8 => {
            let val: u32 = 70000;
            let result = u32_as_u8(val);
            solana_msg::msg!("u32 as u8: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsU16 => {
            let val: u32 = 70000;
            let result = u32_as_u16(val);
            solana_msg::msg!("u32 as u16: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsU64 => {
            let val: u32 = 70000;
            let result = u32_as_u64(val);
            solana_msg::msg!("u32 as u64: {}", result);
        }
        CuLibraryInstruction::ConversionsU32AsUsize => {
            let val: u32 = 70000;
            let result = u32_as_usize(val);
            solana_msg::msg!("u32 as usize: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsU8 => {
            let val: u64 = 70000;
            let result = u64_as_u8(val);
            solana_msg::msg!("u64 as u8: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsU16 => {
            let val: u64 = 70000;
            let result = u64_as_u16(val);
            solana_msg::msg!("u64 as u16: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsU32 => {
            let val: u64 = 70000;
            let result = u64_as_u32(val);
            solana_msg::msg!("u64 as u32: {}", result);
        }
        CuLibraryInstruction::ConversionsU64AsUsize => {
            let val: u64 = 70000;
            let result = u64_as_usize(val);
            solana_msg::msg!("u64 as usize: {}", result);
        }
        // Option handling
        CuLibraryInstruction::OptionCheckedAddU8Unwrap => {
            // Get u8 value from instruction data (3rd byte if available, otherwise use 254)
            let val = if instruction_data.len() > 2 {
                instruction_data[2]
            } else {
                254u8 // Value that will overflow when adding 1
            };
            let result = checked_add_u8_unwrap(val);
            solana_msg::msg!("option checked_add unwrap: {}", result);
        }
        CuLibraryInstruction::OptionCheckedAddU8OkOr => {
            let val = if instruction_data.len() > 2 {
                instruction_data[2]
            } else {
                254u8
            };
            let result = checked_add_u8_ok_or(val);
            solana_msg::msg!("option checked_add ok_or: {:?}", result.is_ok());
        }
        CuLibraryInstruction::OptionCheckedAddU8OkOrElse => {
            let val = if instruction_data.len() > 2 {
                instruction_data[2]
            } else {
                254u8
            };
            let result = checked_add_u8_ok_or_else(val);
            solana_msg::msg!("option checked_add ok_or_else: {:?}", result.is_ok());
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
