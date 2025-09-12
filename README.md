# CU Library Benchmarks

Benchmark results for Solana runtime operations:

**Note:** The `#[profile]` macro adds ~5-6 CU overhead to each measurement.

## Table of Contents

● **[Baseline](#baseline)**  
● **[Access](#access)**  
● **[Account](#account)**  
● **[Add](#add)**  
● **[Array](#array)**  
● **[Arrayvec](#arrayvec)**  
● **[Checked](#checked)**  
● **[Conversions](#conversions)**  
● **[Cpi](#cpi)**  
● **[Option](#option)**  
● **[Partial](#partial)**  
● **[Pinocchio](#pinocchio)**  
● **[Saturating](#saturating)**  
● **[Solana](#solana)**  
● **[Std](#std)**  
● **[Sub](#sub)**  
● **[Vec](#vec)**  

## Baseline

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [baseline_empty_function](https://github.com/Lightprotocol/cu-library/blob/master/src/lib.rs#L188)                                               | 6           |

## Access

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [access_array_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L4)                                 | 9           |
| [access_array_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L9)                                   | 6           |
| [access_array_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L14)                            | 6           |
| [access_array_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L19)                           | 9           |
| [access_array_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L4)                               | 9           |
| [access_array_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L9)                                 | 6           |
| [access_array_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L14)                          | 6           |
| [access_array_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L19)                         | 9           |
| [access_vec_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L4)                                     | 9           |
| [access_vec_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L9)                                       | 8           |
| [access_vec_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L14)                                | 8           |
| [access_vec_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L19)                               | 9           |
| [access_vec_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L4)                                   | 9           |
| [access_vec_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L9)                                     | 8           |
| [access_vec_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L14)                              | 8           |
| [access_vec_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L19)                             | 9           |

## Account

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [account_info_key](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_key.rs#L5)                                   | 10          |
| [account_info_owner](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_owner.rs#L5)                               | 10          |
| [account_info_is_signer](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L5)                          | 13          |
| [account_info_is_writable](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L10)                       | 12          |
| [account_info_executable](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L15)                        | 13          |
| [account_info_data_len](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_data.rs#L5)                             | 10          |
| [account_info_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_data.rs#L10)                            | 10          |
| [account_info_data_is_empty](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L20)                     | 13          |
| [account_info_is_owned_by](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_ownership.rs#L5)                     | 31          |
| [account_info_assign](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_ownership.rs#L10)                         | 17          |
| [account_info_is_borrowed](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L5)                       | 13          |
| [account_info_borrow_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L10)        | 10          |
| [account_info_borrow_mut_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L15)    | 10          |
| [account_info_borrow_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L20)            | 9           |
| [account_info_borrow_mut_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L25)        | 9           |
| [account_info_try_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L30)              | 16          |
| [account_info_try_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L36)          | 11          |
| [account_info_can_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L42)              | 9           |
| [account_info_can_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L47)          | 9           |
| [account_info_try_borrow_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L52)                  | 15          |
| [account_info_try_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L58)              | 12          |
| [account_info_can_borrow_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L64)                  | 9           |
| [account_info_can_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L69)              | 9           |
| [account_info_realloc](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_realloc.rs#L5)                           | 17          |

## Add

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [add_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L4)                                           | 7           |
| [add_assign_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L12)                                         | 7           |
| [add_assign_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L20)                                         | 7           |
| [add_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L28)                                         | 7           |
| [add_assign_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L36)                                        | 9           |

## Array

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [array_new](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_new.rs#L4)                                                   | 10          |
| [array_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_with_capacity.rs#L4)                            | 8           |
| [array_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_with_capacity.rs#L9)                           | 27          |
| [array_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L5)                                          | 9           |
| [array_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L12)                                        | 16          |
| [array_assign_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L19)                                     | 108         |
| [array_assign_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L26)                                      | 9           |
| [array_assign_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L35)                                     | 16          |
| [array_assign_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L44)                                  | 108         |

## Arrayvec

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [arrayvec_u8_new](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_new.rs#L5)                                            | 7           |
| [arrayvec_push_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L20)                                     | 50          |
| [arrayvec_push_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L13)                                        | 43          |
| [arrayvec_push_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L6)                                          | 25          |
| [arrayvec_u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_with_capacity.rs#L5)                     | 7           |
| [arrayvec_u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_with_capacity.rs#L10)                   | 7           |
| [arrayvec_push_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L27)                                      | 10          |
| [arrayvec_push_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L36)                                     | 17          |
| [arrayvec_push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L45)                                  | 88          |

## Checked

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [checked_add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L4)                                     | 7           |
| [checked_add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L11)                                   | 7           |
| [checked_add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L18)                                   | 8           |
| [checked_add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L25)                                   | 8           |
| [checked_add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L32)                                  | 10          |
| [checked_sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L4)                                     | 7           |
| [checked_sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L11)                                   | 7           |
| [checked_sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L18)                                   | 8           |
| [checked_sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L25)                                   | 8           |
| [checked_sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L32)                                  | 10          |
| [checked_mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L4)                                     | 7           |
| [checked_mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L11)                                   | 7           |
| [checked_mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L18)                                   | 8           |
| [checked_mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L25)                                   | 8           |
| [checked_mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L32)                                  | 10          |
| [checked_div_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L4)                                     | 7           |
| [checked_div_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L11)                                   | 7           |
| [checked_div_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L18)                                   | 8           |
| [checked_div_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L25)                                   | 8           |
| [checked_div_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L32)                                  | 10          |

## Conversions

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [conversions_try_into_slice_to_array_32_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/slice_to_array.rs#L7)    | 15          |
| [conversions_try_into_slice_to_array_32_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/slice_to_array.rs#L12)  | 6           |
| [conversions_try_into_usize_to_u64_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/from_usize.rs#L6)             | 7           |
| [conversions_try_into_usize_to_u64_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/from_usize.rs#L11)           | 6           |
| [conversions_try_into_u64_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L6)               | 7           |
| [conversions_try_into_u64_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L11)             | 6           |
| [conversions_try_into_u32_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L17)              | 7           |
| [conversions_try_into_u32_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L22)             | 6           |
| [conversions_try_into_u16_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L28)              | 7           |
| [conversions_try_into_u16_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L33)             | 6           |
| [conversions_try_into_u8_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L39)               | 7           |
| [conversions_try_into_u8_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L44)              | 6           |
| [conversions_u8_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L5)                                   | 7           |
| [conversions_u8_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L10)                                  | 7           |
| [conversions_u8_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L15)                                  | 7           |
| [conversions_u8_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L20)                                | 7           |
| [conversions_u16_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L5)                                  | 7           |
| [conversions_u16_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L10)                                | 7           |
| [conversions_u16_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L15)                                | 7           |
| [conversions_u16_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L20)                              | 7           |
| [conversions_u32_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L5)                                  | 7           |
| [conversions_u32_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L10)                                | 7           |
| [conversions_u32_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L15)                                | 7           |
| [conversions_u32_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L20)                              | 7           |
| [conversions_u64_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L5)                                  | 7           |
| [conversions_u64_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L10)                                | 7           |
| [conversions_u64_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L15)                                | 7           |
| [conversions_u64_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L20)                              | 7           |

## Cpi

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [cpi_account_meta_array_10](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L5)                                    | 6           |
| [cpi_account_info_array_10_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L61)                               | 6           |
| [cpi_account_info_array_10_clone](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L77)                             | 6           |
| [cpi_account_info_array_10_move](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L93)                              | 6           |
| [cpi_arrayvec_push_account_meta_10](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L6)                          | 182         |
| [cpi_arrayvec_push_account_info_10_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L19)                     | 6           |
| [cpi_arrayvec_push_account_info_10_clone](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L30)                   | 8           |
| [cpi_arrayvec_push_account_info_10_move](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L41)                    | 8           |
| [cpi_account_meta_array_10_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L5)                           | 201         |
| [cpi_account_info_array_10_ref_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L28)                      | 6           |
| [cpi_account_info_array_10_clone_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L41)                    | 8           |
| [cpi_account_info_array_10_move_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L54)                     | 8           |

## Option

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [option_checked_add_u8_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L7)                      | 7           |
| [option_checked_add_u8_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L13)                      | 6           |
| [option_checked_add_u8_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L19)                 | 6           |
| [option_checked_add_u8_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L25)          | 8           |
| [option_checked_add_u8_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L31)                  | 8           |
| [option_slice_get_array_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L7)                       | 15          |
| [option_slice_get_array_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L13)                       | 6           |
| [option_slice_get_array_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L19)                  | 6           |
| [option_slice_get_array_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L25)           | 15          |
| [option_slice_get_array_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L31)                   | 15          |
| [option_pubkey_ref_map_deref](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_pubkey_ref.rs#L7)                        | 6           |
| [option_pubkey_as_ref_map_convert](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_pubkey_ref.rs#L12)                  | 6           |
| [option_if_let_some_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L7)                                  | 6           |
| [option_if_let_some_array](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L16)                              | 15          |
| [option_if_let_some_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L25)                             | 15          |
| [option_if_let_some_array_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L34)                          | 6           |

## Partial

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [partial_eq_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L4)                              | 7           |
| [partial_eq_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L9)                             | 7           |
| [partial_eq_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L14)                            | 7           |
| [partial_eq_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L19)                            | 7           |
| [partial_eq_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L24)                           | 7           |
| [partial_eq_array_u8_32_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L5)                     | 7           |
| [partial_eq_array_u8_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L11)                        | 6           |
| [partial_eq_array_u16_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L16)                       | 7           |
| [partial_eq_array_u32_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L21)                       | 7           |
| [partial_eq_array_u64_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L26)                       | 7           |
| [partial_eq_u8_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L5)                                 | 9           |
| [partial_eq_u16_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L10)                               | 9           |
| [partial_eq_u32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L15)                               | 10          |
| [partial_eq_u64_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L20)                               | 10          |
| [partial_eq_u128_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L25)                              | 13          |
| [partial_eq_array_u8_32_neq_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L32)                   | 35          |
| [partial_eq_array_u8_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L38)                       | 31          |
| [partial_eq_array_u8_32_neq_deref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L44)                 | 35          |
| [partial_eq_array_u16_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L49)                      | 33          |
| [partial_eq_array_u32_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L54)                      | 34          |
| [partial_eq_array_u64_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L59)                      | 34          |

## Pinocchio

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [pinocchio_msg10_chars](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/msg.rs#L5)                                     | 110         |
| [pinocchio_sysvar_rent_exemption_165](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/sysvar_rent.rs#L5)               | 151         |
| [pinocchio_clock_get_slot](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/sysvar_clock.rs#L5)                         | 171         |

## Saturating

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [saturating_add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L4)                            | 7           |
| [saturating_add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L11)                          | 7           |
| [saturating_add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L18)                          | 7           |
| [saturating_add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L25)                          | 7           |
| [saturating_add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L32)                         | 8           |
| [saturating_sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L4)                            | 7           |
| [saturating_sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L11)                          | 7           |
| [saturating_sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L18)                          | 7           |
| [saturating_sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L25)                          | 7           |
| [saturating_sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L32)                         | 8           |
| [saturating_mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L4)                            | 7           |
| [saturating_mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L11)                          | 7           |
| [saturating_mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L18)                          | 7           |
| [saturating_mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L25)                          | 7           |
| [saturating_mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L32)                         | 8           |

## Solana

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [solana_msg10_chars](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/msg.rs#L5)                                           | 110         |
| [solana_msg_program_id](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/msg_program_id.rs#L5)                             | 6954        |
| [solana_pubkey_new_from_array](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/pubkey_new_from_array.rs#L5)               | 15          |
| [solana_pubkey_to_bytes](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/pubkey_to_bytes.rs#L6)                           | 15          |

## Std

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [std_add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L4)                                                 | 6           |
| [std_add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L11)                                               | 6           |
| [std_add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L18)                                               | 6           |
| [std_add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L25)                                               | 6           |
| [std_add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L32)                                              | 6           |
| [std_sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L4)                                                 | 6           |
| [std_sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L11)                                               | 6           |
| [std_sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L18)                                               | 6           |
| [std_sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L25)                                               | 6           |
| [std_sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L32)                                              | 6           |
| [std_mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L4)                                                 | 6           |
| [std_mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L11)                                               | 6           |
| [std_mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L18)                                               | 6           |
| [std_mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L25)                                               | 6           |
| [std_mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L32)                                              | 6           |
| [std_div_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L4)                                                 | 6           |
| [std_div_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L11)                                               | 6           |
| [std_div_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L18)                                               | 6           |
| [std_div_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L25)                                               | 6           |
| [std_div_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L32)                                              | 6           |

## Sub

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [sub_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L4)                                           | 7           |
| [sub_assign_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L12)                                         | 7           |
| [sub_assign_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L20)                                         | 7           |
| [sub_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L28)                                         | 7           |
| [sub_assign_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L36)                                        | 9           |

## Vec

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [vec_u8_new](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_new.rs#L4)                                                      | 9           |
| [vec_u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_with_capacity.rs#L4)                               | 113         |
| [vec_u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_with_capacity.rs#L9)                              | 113         |
| [vec_push_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L5)                                                    | 73          |
| [vec_push_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L12)                                                  | 78          |
| [vec_push_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L19)                                               | 85          |
| [vec_push_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L26)                                                | 202         |
| [vec_push_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L35)                                               | 313         |
| [vec_push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L44)                                            | 380         |
| [vec_push_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L53)                                     | 120         |
| [vec_push_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L60)                                    | 120         |
| [vec_push_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L67)                                 | 128         |
| [vec_push_10_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L74)                                  | 153         |
| [vec_push_10_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L83)                                 | 149         |
| [vec_push_10_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L92)                              | 218         |

