use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::instructions::discriminator::CuLibraryInstruction;
use crate::{collections, solana_ops};

use crate::collections::array::array_u64_10::{
    array_u64_10_get, array_u64_10_get_ok_or, array_u64_10_if_let_get, array_u64_10_index,
};
use crate::collections::array::array_u8_32::{
    array_u8_32_get, array_u8_32_get_ok_or, array_u8_32_if_let_get, array_u8_32_index,
};
use crate::collections::vec::vec_u64_10::{
    vec_u64_10_get, vec_u64_10_get_ok_or, vec_u64_10_if_let_get, vec_u64_10_index,
};
use crate::collections::vec::vec_u8_32::{
    vec_u8_32_get, vec_u8_32_get_ok_or, vec_u8_32_if_let_get, vec_u8_32_index,
};

use crate::option::option_checked_add::{
    checked_add_u8_unwrap_or, checked_add_u8_unwrap_or_default,
};
use crate::option::option_if_let::{
    if_let_some_array, if_let_some_array_ref, if_let_some_pubkey, if_let_some_u8,
};
use crate::option::option_pubkey_ref::{pubkey_as_ref_map_convert, pubkey_ref_map_deref};
use crate::option::option_slice_get::{
    slice_get_array_ok_or, slice_get_array_ok_or_else, slice_get_array_unwrap,
    slice_get_array_unwrap_or, slice_get_array_unwrap_or_default,
};

use crate::serialization::compressed_account_info::{
    bincode_deserialize, borsh1_deserialize, borsh_deserialize, rkyv_zero_copy_deserialize,
    serialize_compressed_account_info, serialize_compressed_account_info_bincode,
    serialize_compressed_account_info_borsh1, serialize_compressed_account_info_rkyv,
    serialize_compressed_account_info_wincode, serialize_compressed_account_info_wincode_shortvec,
    wincode_deserialize, wincode_shortvec_deserialize, zero_copy_deserialize,
};

#[inline(never)]
pub fn process_instruction_200_299(
    instruction: CuLibraryInstruction,
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    match instruction {
        CuLibraryInstruction::OptionCheckedAddU8UnwrapOrDefault => {
            let val = if instruction_data.len() > 2 {
                instruction_data[2]
            } else {
                255u8 // Will overflow
            };
            let result = checked_add_u8_unwrap_or_default(val);
            solana_msg::msg!("option checked_add unwrap_or_default: {}", result);
        }
        CuLibraryInstruction::OptionCheckedAddU8UnwrapOr => {
            let val = if instruction_data.len() > 2 {
                instruction_data[2]
            } else {
                255u8 // Will overflow
            };
            let result = checked_add_u8_unwrap_or(val);
            solana_msg::msg!("option checked_add unwrap_or: {}", result);
        }
        CuLibraryInstruction::OptionSliceGetArrayUnwrap => {
            let arrays: [[u8; 32]; 2] = [*program_id, [2u8; 32]];
            let result = slice_get_array_unwrap(&arrays);
            solana_msg::msg!("option slice get unwrap: {:?}", result[0]);
        }
        CuLibraryInstruction::OptionSliceGetArrayOkOr => {
            let arrays: [[u8; 32]; 2] = [*program_id, [2u8; 32]];
            let result = slice_get_array_ok_or(&arrays);
            solana_msg::msg!("option slice get ok_or: {:?}", result.is_ok());
        }
        CuLibraryInstruction::OptionSliceGetArrayOkOrElse => {
            let arrays: [[u8; 32]; 2] = [*program_id, [2u8; 32]];
            let result = slice_get_array_ok_or_else(&arrays);
            solana_msg::msg!("option slice get ok_or_else: {:?}", result.is_ok());
        }
        CuLibraryInstruction::OptionSliceGetArrayUnwrapOrDefault => {
            let arrays: [[u8; 32]; 2] = [*program_id, [2u8; 32]];
            let result = slice_get_array_unwrap_or_default(&arrays);
            solana_msg::msg!("option slice get unwrap_or_default: {:?}", result[0]);
        }
        CuLibraryInstruction::OptionSliceGetArrayUnwrapOr => {
            let arrays: [[u8; 32]; 2] = [*program_id, [2u8; 32]];
            let result = slice_get_array_unwrap_or(&arrays);
            solana_msg::msg!("option slice get unwrap_or: {:?}", result[0]);
        }
        CuLibraryInstruction::OptionPubkeyRefMapDeref => {
            let pubkey_option: Option<&Pubkey> = Some(program_id);
            let result = pubkey_ref_map_deref(pubkey_option);
            solana_msg::msg!("option pubkey ref map deref: {:?}", result.is_some());
        }
        CuLibraryInstruction::OptionPubkeyAsRefMapConvert => {
            let pubkey_bytes: Option<[u8; 32]> = Some(*program_id);
            let result = pubkey_as_ref_map_convert(pubkey_bytes);
            solana_msg::msg!("option pubkey as_ref map convert: {:?}", result.is_some());
        }
        CuLibraryInstruction::OptionIfLetSomeU8 => {
            let val = if instruction_data.len() > 2 {
                instruction_data[2]
            } else {
                42u8
            };
            let option = Some(val);
            let result = if_let_some_u8(option);
            solana_msg::msg!("option if let some u8: {}", result);
        }
        CuLibraryInstruction::OptionIfLetSomeArray => {
            let array: [u8; 32] = *program_id;
            let option = Some(array);
            let result = if_let_some_array(option);
            solana_msg::msg!("option if let some array: {:?}", result[0]);
        }
        CuLibraryInstruction::OptionIfLetSomePubkey => {
            let pubkey = *program_id;
            let option = Some(pubkey);
            let result = if_let_some_pubkey(option);
            solana_msg::msg!("option if let some pubkey: {:?}", result[0]);
        }
        CuLibraryInstruction::OptionIfLetSomeArrayRef => {
            let array: [u8; 32] = *program_id;
            let option = Some(&array);
            let result = if_let_some_array_ref(option);
            solana_msg::msg!("option if let some array ref: {:?}", result[0]);
        }
        CuLibraryInstruction::SolanaPubkeyToBytes => {
            let solana_pubkey = solana_pubkey::Pubkey::from(*program_id);
            let res = solana_ops::pubkey_to_bytes::pubkey_to_bytes(&solana_pubkey);
            solana_msg::msg!("pubkey to_bytes: {:?}", res[0]);
        }
        CuLibraryInstruction::ArrayU8_32Index => {
            let array: [u8; 32] = [1; 32];
            let res = array_u8_32_index(&array);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::ArrayU8_32Get => {
            let array: [u8; 32] = [1; 32];
            let res = array_u8_32_get(&array);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::ArrayU8_32GetOkOr => {
            let array: [u8; 32] = [1; 32];
            let res = array_u8_32_get_ok_or(&array);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::ArrayU8_32IfLetGet => {
            let array: [u8; 32] = [1; 32];
            let res = array_u8_32_if_let_get(&array);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::ArrayU64_10Index => {
            let array: [u64; 10] = [100; 10];
            let res = array_u64_10_index(&array);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::ArrayU64_10Get => {
            let array: [u64; 10] = [100; 10];
            let res = array_u64_10_get(&array);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::ArrayU64_10GetOkOr => {
            let array: [u64; 10] = [100; 10];
            let res = array_u64_10_get_ok_or(&array);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::ArrayU64_10IfLetGet => {
            let array: [u64; 10] = [100; 10];
            let res = array_u64_10_if_let_get(&array);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::VecU8_32Index => {
            let vec: Vec<u8> = vec![1; 32];
            let res = vec_u8_32_index(&vec);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::VecU8_32Get => {
            let vec: Vec<u8> = vec![1; 32];
            let res = vec_u8_32_get(&vec);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::VecU8_32GetOkOr => {
            let vec: Vec<u8> = vec![1; 32];
            let res = vec_u8_32_get_ok_or(&vec);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::VecU8_32IfLetGet => {
            let vec: Vec<u8> = vec![1; 32];
            let res = vec_u8_32_if_let_get(&vec);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::VecU64_10Index => {
            let vec: Vec<u64> = vec![100; 10];
            let res = vec_u64_10_index(&vec);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::VecU64_10Get => {
            let vec: Vec<u64> = vec![100; 10];
            let res = vec_u64_10_get(&vec);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::VecU64_10GetOkOr => {
            let vec: Vec<u64> = vec![100; 10];
            let res = vec_u64_10_get_ok_or(&vec);
            solana_msg::msg!("{}", res.unwrap_or(&0));
        }
        CuLibraryInstruction::VecU64_10IfLetGet => {
            let vec: Vec<u64> = vec![100; 10];
            let res = vec_u64_10_if_let_get(&vec);
            solana_msg::msg!("{}", res);
        }
        CuLibraryInstruction::SerializationCompressedAccountInfoBorshDeserialize => {
            let data = serialize_compressed_account_info();
            let res = borsh_deserialize(data.as_slice())?;
            solana_msg::msg!("Borsh deserialized: {:?}", res.address.is_some());
        }
        CuLibraryInstruction::SerializationCompressedAccountInfoZeroCopyDeserialize => {
            let data = serialize_compressed_account_info();
            let res = zero_copy_deserialize(data.as_slice())?;
            solana_msg::msg!("Zerocopy deserialized: {:?}", res.address.is_some());
        }
        CuLibraryInstruction::SerializationCompressedAccountInfoWincodeDeserialize => {
            let data = serialize_compressed_account_info_wincode();
            let res = wincode_deserialize(data.as_slice())?;
            solana_msg::msg!("Wincode deserialized: {:?}", res.address.is_some());
        }
        CuLibraryInstruction::SerializationCompressedAccountInfoBincodeDeserialize => {
            let data = serialize_compressed_account_info_bincode();
            let res = bincode_deserialize(data.as_slice())?;
            solana_msg::msg!("Bincode deserialized: {:?}", res.address.is_some());
        }
        CuLibraryInstruction::SerializationCompressedAccountInfoBorsh1Deserialize => {
            let data = serialize_compressed_account_info_borsh1();
            let res = borsh1_deserialize(data.as_slice())?;
            solana_msg::msg!("Borsh1 deserialized: {:?}", res.address.is_some());
        }
        CuLibraryInstruction::SerializationCompressedAccountInfoRkyvZeroCopyDeserialize => {
            let data = serialize_compressed_account_info_rkyv();
            let res = rkyv_zero_copy_deserialize(data.as_slice())?;
            solana_msg::msg!("Rkyv zero-copy deserialized: {:?}", res.address.is_some());
        }
        CuLibraryInstruction::SerializationCompressedAccountInfoWincodeShortVecDeserialize => {
            let data = serialize_compressed_account_info_wincode_shortvec();
            let res = wincode_shortvec_deserialize(data.as_slice())?;
            solana_msg::msg!(
                "Wincode short-vec deserialized: {:?}",
                res.address.is_some()
            );
        }
        CuLibraryInstruction::ArrayvecGetFirstPubkey => {
            let vec = collections::arrayvec::access::create_populated_vec(program_id);
            let res = collections::arrayvec::access::get_first_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::ArrayvecGet10thPubkey => {
            let vec = collections::arrayvec::access::create_populated_vec(program_id);
            let res = collections::arrayvec::access::get_10th_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::ArrayvecFindPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec =
                collections::arrayvec::access::create_vec_target_first(program_id, &other_key);
            let res = collections::arrayvec::access::find_pubkey_1_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::ArrayvecFindPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::arrayvec::access::create_vec_target_last(program_id, &other_key);
            let res = collections::arrayvec::access::find_pubkey_10_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::ArrayvecPositionPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec =
                collections::arrayvec::access::create_vec_target_first(program_id, &other_key);
            let res = collections::arrayvec::access::position_pubkey_1_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::ArrayvecPositionPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::arrayvec::access::create_vec_target_last(program_id, &other_key);
            let res = collections::arrayvec::access::position_pubkey_10_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::ArrayvecUpdateIndex => {
            let mut vec = collections::arrayvec::access::create_populated_vec(program_id);
            collections::arrayvec::access::update_index(&mut vec, 5, program_id);
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::ArrayvecUpdateGetMut => {
            let mut vec = collections::arrayvec::access::create_populated_vec(program_id);
            collections::arrayvec::access::update_get_mut(&mut vec, 5, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::ArrayvecUpdateIterMutFind => {
            let mut vec = collections::arrayvec::access::create_populated_vec(program_id);
            let target = Pubkey::from([1u8; 32]);
            collections::arrayvec::access::update_iter_mut_find(&mut vec, &target, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        // Tinyvec benchmarks
        CuLibraryInstruction::TinyvecU8New => {
            let res = collections::tinyvec::vec_new::u8_new();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::TinyvecU8WithCapacity10 => {
            let res = collections::tinyvec::vec_with_capacity::u8_with_capacity_10();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::TinyvecU8WithCapacity100 => {
            let res = collections::tinyvec::vec_with_capacity::u8_with_capacity_100();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::TinyvecPushU8 => {
            let mut vec = collections::tinyvec::vec_push::create_empty_u8_vec();
            collections::tinyvec::vec_push::push_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecPushU64 => {
            let mut vec = collections::tinyvec::vec_push::create_empty_u64_vec();
            collections::tinyvec::vec_push::push_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecPushPubkey => {
            let mut vec = collections::tinyvec::vec_push::create_empty_pubkey_vec();
            collections::tinyvec::vec_push::push_pubkey(&mut vec, program_id);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecPush10U8 => {
            let mut vec = collections::tinyvec::vec_push::create_empty_u8_vec();
            collections::tinyvec::vec_push::push_10_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecPush10U64 => {
            let mut vec = collections::tinyvec::vec_push::create_empty_u64_vec();
            collections::tinyvec::vec_push::push_10_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecPush10Pubkey => {
            let mut vec = collections::tinyvec::vec_push::create_empty_pubkey_vec();
            collections::tinyvec::vec_push::push_10_pubkey(&mut vec, program_id)?;
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecGetFirstPubkey => {
            let vec = collections::tinyvec::access::create_populated_vec(program_id);
            let res = collections::tinyvec::access::get_first_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::TinyvecGet10thPubkey => {
            let vec = collections::tinyvec::access::create_populated_vec(program_id);
            let res = collections::tinyvec::access::get_10th_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::TinyvecFindPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::tinyvec::access::create_vec_target_first(program_id, &other_key);
            let res = collections::tinyvec::access::find_pubkey_1_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::TinyvecFindPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::tinyvec::access::create_vec_target_last(program_id, &other_key);
            let res = collections::tinyvec::access::find_pubkey_10_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::TinyvecPositionPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::tinyvec::access::create_vec_target_first(program_id, &other_key);
            let res = collections::tinyvec::access::position_pubkey_1_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::TinyvecPositionPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::tinyvec::access::create_vec_target_last(program_id, &other_key);
            let res = collections::tinyvec::access::position_pubkey_10_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::TinyvecUpdateIndex => {
            let mut vec = collections::tinyvec::access::create_populated_vec(program_id);
            collections::tinyvec::access::update_index(&mut vec, 5, program_id);
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecUpdateGetMut => {
            let mut vec = collections::tinyvec::access::create_populated_vec(program_id);
            collections::tinyvec::access::update_get_mut(&mut vec, 5, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecUpdateIterMutFind => {
            let mut vec = collections::tinyvec::access::create_populated_vec(program_id);
            let target = Pubkey::from([1u8; 32]);
            collections::tinyvec::access::update_iter_mut_find(&mut vec, &target, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        // Heapless benchmarks
        CuLibraryInstruction::HeaplessU8New => {
            let res = collections::heapless::vec_new::u8_new();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::HeaplessU8WithCapacity10 => {
            let res = collections::heapless::vec_with_capacity::u8_with_capacity_10();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::HeaplessU8WithCapacity100 => {
            let res = collections::heapless::vec_with_capacity::u8_with_capacity_100();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::HeaplessPushU8 => {
            let mut vec = collections::heapless::vec_push::create_empty_u8_vec();
            collections::heapless::vec_push::push_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessPushU64 => {
            let mut vec = collections::heapless::vec_push::create_empty_u64_vec();
            collections::heapless::vec_push::push_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessPushPubkey => {
            let mut vec = collections::heapless::vec_push::create_empty_pubkey_vec();
            collections::heapless::vec_push::push_pubkey(&mut vec, program_id);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessPush10U8 => {
            let mut vec = collections::heapless::vec_push::create_empty_u8_vec();
            collections::heapless::vec_push::push_10_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessPush10U64 => {
            let mut vec = collections::heapless::vec_push::create_empty_u64_vec();
            collections::heapless::vec_push::push_10_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessPush10Pubkey => {
            let mut vec = collections::heapless::vec_push::create_empty_pubkey_vec();
            collections::heapless::vec_push::push_10_pubkey(&mut vec, program_id)?;
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessGetFirstPubkey => {
            let vec = collections::heapless::access::create_populated_vec(program_id);
            let res = collections::heapless::access::get_first_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::HeaplessGet10thPubkey => {
            let vec = collections::heapless::access::create_populated_vec(program_id);
            let res = collections::heapless::access::get_10th_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::HeaplessFindPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec =
                collections::heapless::access::create_vec_target_first(program_id, &other_key);
            let res = collections::heapless::access::find_pubkey_1_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::HeaplessFindPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::heapless::access::create_vec_target_last(program_id, &other_key);
            let res = collections::heapless::access::find_pubkey_10_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::HeaplessPositionPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec =
                collections::heapless::access::create_vec_target_first(program_id, &other_key);
            let res = collections::heapless::access::position_pubkey_1_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::HeaplessPositionPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::heapless::access::create_vec_target_last(program_id, &other_key);
            let res = collections::heapless::access::position_pubkey_10_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::HeaplessUpdateIndex => {
            let mut vec = collections::heapless::access::create_populated_vec(program_id);
            collections::heapless::access::update_index(&mut vec, 5, program_id);
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessUpdateGetMut => {
            let mut vec = collections::heapless::access::create_populated_vec(program_id);
            collections::heapless::access::update_get_mut(&mut vec, 5, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::HeaplessUpdateIterMutFind => {
            let mut vec = collections::heapless::access::create_populated_vec(program_id);
            let target = Pubkey::from([1u8; 32]);
            collections::heapless::access::update_iter_mut_find(&mut vec, &target, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        // Smallvec benchmarks
        CuLibraryInstruction::SmallvecU8New => {
            let res = collections::smallvec::vec_new::u8_new();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::SmallvecU8WithCapacity10 => {
            let res = collections::smallvec::vec_with_capacity::u8_with_capacity_10();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::SmallvecU8WithCapacity128 => {
            let res = collections::smallvec::vec_with_capacity::u8_with_capacity_128();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::SmallvecPushU8 => {
            let mut vec = collections::smallvec::vec_push::create_empty_u8_vec();
            collections::smallvec::vec_push::push_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecPushU64 => {
            let mut vec = collections::smallvec::vec_push::create_empty_u64_vec();
            collections::smallvec::vec_push::push_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecPushPubkey => {
            let mut vec = collections::smallvec::vec_push::create_empty_pubkey_vec();
            collections::smallvec::vec_push::push_pubkey(&mut vec, program_id);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecPush10U8 => {
            let mut vec = collections::smallvec::vec_push::create_empty_u8_vec();
            collections::smallvec::vec_push::push_10_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecPush10U64 => {
            let mut vec = collections::smallvec::vec_push::create_empty_u64_vec();
            collections::smallvec::vec_push::push_10_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecPush10Pubkey => {
            let mut vec = collections::smallvec::vec_push::create_empty_pubkey_vec();
            collections::smallvec::vec_push::push_10_pubkey(&mut vec, program_id)?;
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecGetFirstPubkey => {
            let vec = collections::smallvec::access::create_populated_vec(program_id);
            let res = collections::smallvec::access::get_first_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::SmallvecGet10thPubkey => {
            let vec = collections::smallvec::access::create_populated_vec(program_id);
            let res = collections::smallvec::access::get_10th_pubkey(&vec)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::SmallvecFindPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec =
                collections::smallvec::access::create_vec_target_first(program_id, &other_key);
            let res = collections::smallvec::access::find_pubkey_1_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::SmallvecFindPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::smallvec::access::create_vec_target_last(program_id, &other_key);
            let res = collections::smallvec::access::find_pubkey_10_iters(&vec, program_id)?;
            solana_msg::msg!("pubkey: {:?}", res);
        }
        CuLibraryInstruction::SmallvecPositionPubkey1Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec =
                collections::smallvec::access::create_vec_target_first(program_id, &other_key);
            let res = collections::smallvec::access::position_pubkey_1_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::SmallvecPositionPubkey10Iters => {
            let other_key = Pubkey::from([1u8; 32]);
            let vec = collections::smallvec::access::create_vec_target_last(program_id, &other_key);
            let res = collections::smallvec::access::position_pubkey_10_iters(&vec, program_id);
            solana_msg::msg!("position: {:?}", res);
        }
        CuLibraryInstruction::SmallvecUpdateIndex => {
            let mut vec = collections::smallvec::access::create_populated_vec(program_id);
            collections::smallvec::access::update_index(&mut vec, 5, program_id);
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecUpdateGetMut => {
            let mut vec = collections::smallvec::access::create_populated_vec(program_id);
            collections::smallvec::access::update_get_mut(&mut vec, 5, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::SmallvecUpdateIterMutFind => {
            let mut vec = collections::smallvec::access::create_populated_vec(program_id);
            let target = Pubkey::from([1u8; 32]);
            collections::smallvec::access::update_iter_mut_find(&mut vec, &target, program_id)?;
            solana_msg::msg!("updated vec: {:?}", vec.as_slice());
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
