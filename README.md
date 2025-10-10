# CU Library Benchmarks

Benchmark results for Solana runtime operations.

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
● **[Serialize](#serialize)**
● **[Solana](#solana)**
● **[Std](#std)**
● **[Sub](#sub)**
● **[Vec](#vec)**

## Definitions

- **CU Consumed**: Total compute units consumed by the profiled function
- **CU Adjusted**: Actual operation cost with baseline profiling overhead subtracted (CU Consumed - Baseline CU)
- **Baseline CU**: CU consumed by an empty profiled function (`#[profile]` macro overhead)

## Baseline

| Function                                                                                                                                         | CU Consumed |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [baseline_empty_function](https://github.com/Lightprotocol/cu-library/blob/master/src/lib.rs#L189)                                               | 6           |

## Access

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [access_array_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L4)                                 | 9           | 3           |
| [access_array_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L9)                                   | 6           | 0           |
| [access_array_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L14)                            | 6           | 0           |
| [access_array_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L19)                           | 9           | 3           |
| [access_array_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L4)                               | 9           | 3           |
| [access_array_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L9)                                 | 6           | 0           |
| [access_array_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L14)                          | 6           | 0           |
| [access_array_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L19)                         | 9           | 3           |
| [access_vec_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L4)                                     | 9           | 3           |
| [access_vec_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L9)                                       | 8           | 2           |
| [access_vec_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L14)                                | 8           | 2           |
| [access_vec_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L19)                               | 9           | 3           |
| [access_vec_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L4)                                   | 9           | 3           |
| [access_vec_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L9)                                     | 8           | 2           |
| [access_vec_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L14)                              | 8           | 2           |
| [access_vec_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L19)                             | 9           | 3           |

## Account

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [account_info_key](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_key.rs#L5)                                   | 10          | 4           |
| [account_info_owner](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_owner.rs#L5)                               | 10          | 4           |
| [account_info_is_signer](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L5)                          | 13          | 7           |
| [account_info_is_writable](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L10)                       | 12          | 6           |
| [account_info_executable](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L15)                        | 13          | 7           |
| [account_info_data_len](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_data.rs#L5)                             | 10          | 4           |
| [account_info_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_data.rs#L10)                            | 10          | 4           |
| [account_info_data_is_empty](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L20)                     | 13          | 7           |
| [account_info_is_owned_by](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_ownership.rs#L5)                     | 31          | 25          |
| [account_info_assign](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_ownership.rs#L10)                         | 17          | 11          |
| [account_info_is_borrowed](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L5)                       | 13          | 7           |
| [account_info_borrow_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L10)        | 10          | 4           |
| [account_info_borrow_mut_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L15)    | 10          | 4           |
| [account_info_borrow_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L20)            | 9           | 3           |
| [account_info_borrow_mut_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L25)        | 9           | 3           |
| [account_info_try_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L30)              | 16          | 10          |
| [account_info_try_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L36)          | 11          | 5           |
| [account_info_can_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L42)              | 9           | 3           |
| [account_info_can_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L47)          | 9           | 3           |
| [account_info_try_borrow_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L52)                  | 15          | 9           |
| [account_info_try_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L58)              | 12          | 6           |
| [account_info_can_borrow_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L64)                  | 9           | 3           |
| [account_info_can_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L69)              | 9           | 3           |
| [account_info_realloc](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_realloc.rs#L5)                           | 17          | 11          |

## Add

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [add_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L4)                                           | 7           | 1           |
| [add_assign_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L12)                                         | 7           | 1           |
| [add_assign_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L20)                                         | 7           | 1           |
| [add_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L28)                                         | 7           | 1           |
| [add_assign_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L36)                                        | 9           | 3           |

## Array

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [array_new](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_new.rs#L4)                                                   | 10          | 4           |
| [array_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_with_capacity.rs#L4)                            | 8           | 2           |
| [array_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_with_capacity.rs#L9)                           | 27          | 21          |
| [array_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L5)                                          | 9           | 3           |
| [array_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L12)                                        | 16          | 10          |
| [array_assign_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L19)                                     | 108         | 102         |
| [array_assign_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L26)                                      | 9           | 3           |
| [array_assign_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L35)                                     | 16          | 10          |
| [array_assign_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L44)                                  | 108         | 102         |

## Arrayvec

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [arrayvec_u8_new](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_new.rs#L5)                                            | 7           | 1           |
| [arrayvec_push_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L20)                                     | 50          | 44          |
| [arrayvec_push_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L13)                                        | 43          | 37          |
| [arrayvec_push_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L6)                                          | 25          | 19          |
| [arrayvec_u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_with_capacity.rs#L5)                     | 7           | 1           |
| [arrayvec_u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_with_capacity.rs#L10)                   | 7           | 1           |
| [arrayvec_push_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L27)                                      | 10          | 4           |
| [arrayvec_push_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L36)                                     | 17          | 11          |
| [arrayvec_push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L45)                                  | 88          | 82          |

## Checked

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [checked_add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L4)                                     | 7           | 1           |
| [checked_add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L11)                                   | 7           | 1           |
| [checked_add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L18)                                   | 8           | 2           |
| [checked_add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L25)                                   | 8           | 2           |
| [checked_add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L32)                                  | 10          | 4           |
| [checked_sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L4)                                     | 7           | 1           |
| [checked_sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L11)                                   | 7           | 1           |
| [checked_sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L18)                                   | 8           | 2           |
| [checked_sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L25)                                   | 8           | 2           |
| [checked_sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L32)                                  | 10          | 4           |
| [checked_mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L4)                                     | 7           | 1           |
| [checked_mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L11)                                   | 7           | 1           |
| [checked_mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L18)                                   | 8           | 2           |
| [checked_mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L25)                                   | 8           | 2           |
| [checked_mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L32)                                  | 10          | 4           |
| [checked_div_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L4)                                     | 7           | 1           |
| [checked_div_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L11)                                   | 7           | 1           |
| [checked_div_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L18)                                   | 8           | 2           |
| [checked_div_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L25)                                   | 8           | 2           |
| [checked_div_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L32)                                  | 10          | 4           |

## Conversions

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [conversions_try_into_slice_to_array_32_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/slice_to_array.rs#L7)    | 15          | 9           |
| [conversions_try_into_slice_to_array_32_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/slice_to_array.rs#L12)  | 6           | 0           |
| [conversions_try_into_usize_to_u64_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/from_usize.rs#L6)             | 7           | 1           |
| [conversions_try_into_usize_to_u64_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/from_usize.rs#L11)           | 6           | 0           |
| [conversions_try_into_u64_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L6)               | 7           | 1           |
| [conversions_try_into_u64_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L11)             | 6           | 0           |
| [conversions_try_into_u32_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L17)              | 7           | 1           |
| [conversions_try_into_u32_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L22)             | 6           | 0           |
| [conversions_try_into_u16_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L28)              | 7           | 1           |
| [conversions_try_into_u16_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L33)             | 6           | 0           |
| [conversions_try_into_u8_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L39)               | 7           | 1           |
| [conversions_try_into_u8_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L44)              | 6           | 0           |
| [conversions_u8_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L5)                                   | 7           | 1           |
| [conversions_u8_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L10)                                  | 7           | 1           |
| [conversions_u8_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L15)                                  | 7           | 1           |
| [conversions_u8_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L20)                                | 7           | 1           |
| [conversions_u16_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L5)                                  | 7           | 1           |
| [conversions_u16_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L10)                                | 7           | 1           |
| [conversions_u16_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L15)                                | 7           | 1           |
| [conversions_u16_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L20)                              | 7           | 1           |
| [conversions_u32_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L5)                                  | 7           | 1           |
| [conversions_u32_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L10)                                | 7           | 1           |
| [conversions_u32_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L15)                                | 7           | 1           |
| [conversions_u32_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L20)                              | 7           | 1           |
| [conversions_u64_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L5)                                  | 7           | 1           |
| [conversions_u64_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L10)                                | 7           | 1           |
| [conversions_u64_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L15)                                | 7           | 1           |
| [conversions_u64_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L20)                              | 7           | 1           |

## Cpi

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [cpi_account_meta_array_10](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L5)                                    | 6           | 0           |
| [cpi_account_info_array_10_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L61)                               | 6           | 0           |
| [cpi_account_info_array_10_clone](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L77)                             | 6           | 0           |
| [cpi_account_info_array_10_move](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L93)                              | 6           | 0           |
| [cpi_arrayvec_push_account_meta_10](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L6)                          | 182         | 176         |
| [cpi_arrayvec_push_account_info_10_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L19)                     | 6           | 0           |
| [cpi_arrayvec_push_account_info_10_clone](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L30)                   | 8           | 2           |
| [cpi_arrayvec_push_account_info_10_move](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L41)                    | 8           | 2           |
| [cpi_account_meta_array_10_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L5)                           | 201         | 195         |
| [cpi_account_info_array_10_ref_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L28)                      | 6           | 0           |
| [cpi_account_info_array_10_clone_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L41)                    | 8           | 2           |
| [cpi_account_info_array_10_move_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L54)                     | 8           | 2           |

## Option

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [option_checked_add_u8_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L7)                      | 7           | 1           |
| [option_checked_add_u8_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L13)                      | 6           | 0           |
| [option_checked_add_u8_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L19)                 | 6           | 0           |
| [option_checked_add_u8_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L25)          | 8           | 2           |
| [option_checked_add_u8_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L31)                  | 8           | 2           |
| [option_slice_get_array_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L7)                       | 15          | 9           |
| [option_slice_get_array_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L13)                       | 6           | 0           |
| [option_slice_get_array_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L19)                  | 6           | 0           |
| [option_slice_get_array_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L25)           | 15          | 9           |
| [option_slice_get_array_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L31)                   | 15          | 9           |
| [option_pubkey_ref_map_deref](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_pubkey_ref.rs#L7)                        | 6           | 0           |
| [option_pubkey_as_ref_map_convert](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_pubkey_ref.rs#L12)                  | 6           | 0           |
| [option_if_let_some_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L7)                                  | 6           | 0           |
| [option_if_let_some_array](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L16)                              | 15          | 9           |
| [option_if_let_some_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L25)                             | 15          | 9           |
| [option_if_let_some_array_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L34)                          | 6           | 0           |

## Partial

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [partial_eq_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L4)                              | 7           | 1           |
| [partial_eq_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L9)                             | 7           | 1           |
| [partial_eq_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L14)                            | 7           | 1           |
| [partial_eq_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L19)                            | 7           | 1           |
| [partial_eq_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L24)                           | 7           | 1           |
| [partial_eq_array_u8_32_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L5)                     | 7           | 1           |
| [partial_eq_array_u8_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L11)                        | 6           | 0           |
| [partial_eq_array_u16_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L16)                       | 7           | 1           |
| [partial_eq_array_u32_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L21)                       | 7           | 1           |
| [partial_eq_array_u64_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L26)                       | 7           | 1           |
| [partial_eq_u8_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L5)                                 | 9           | 3           |
| [partial_eq_u16_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L10)                               | 9           | 3           |
| [partial_eq_u32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L15)                               | 10          | 4           |
| [partial_eq_u64_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L20)                               | 10          | 4           |
| [partial_eq_u128_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L25)                              | 13          | 7           |
| [partial_eq_array_u8_32_neq_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L32)                   | 35          | 29          |
| [partial_eq_array_u8_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L38)                       | 31          | 25          |
| [partial_eq_array_u8_32_neq_deref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L44)                 | 35          | 29          |
| [partial_eq_array_u16_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L49)                      | 33          | 27          |
| [partial_eq_array_u32_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L54)                      | 34          | 28          |
| [partial_eq_array_u64_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L59)                      | 34          | 28          |

## Pinocchio

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [pinocchio_msg10_chars](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/msg.rs#L5)                                     | 110         | 104         |
| [pinocchio_sysvar_rent_exemption_165](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/sysvar_rent.rs#L5)               | 151         | 145         |
| [pinocchio_clock_get_slot](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/sysvar_clock.rs#L5)                         | 171         | 165         |

## Saturating

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [saturating_add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L4)                            | 7           | 1           |
| [saturating_add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L11)                          | 7           | 1           |
| [saturating_add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L18)                          | 7           | 1           |
| [saturating_add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L25)                          | 7           | 1           |
| [saturating_add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L32)                         | 8           | 2           |
| [saturating_sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L4)                            | 7           | 1           |
| [saturating_sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L11)                          | 7           | 1           |
| [saturating_sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L18)                          | 7           | 1           |
| [saturating_sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L25)                          | 7           | 1           |
| [saturating_sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L32)                         | 8           | 2           |
| [saturating_mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L4)                            | 7           | 1           |
| [saturating_mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L11)                          | 7           | 1           |
| [saturating_mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L18)                          | 7           | 1           |
| [saturating_mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L25)                          | 7           | 1           |
| [saturating_mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L32)                         | 8           | 2           |

## Serialize

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [serialize_compressed_account_info_borsh_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L145) | 427         | 421         |
| [serialize_compressed_account_info_zero_copy_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L153) | 130         | 124         |
| [serialize_compressed_account_info_wincode_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L162) | 1408        | 1402        |

## Solana

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [solana_msg10_chars](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/msg.rs#L5)                                           | 110         | 104         |
| [solana_msg_program_id](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/msg_program_id.rs#L5)                             | 6954        | 6948        |
| [solana_pubkey_new_from_array](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/pubkey_new_from_array.rs#L5)               | 15          | 9           |
| [solana_pubkey_to_bytes](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/pubkey_to_bytes.rs#L6)                           | 15          | 9           |

## Std

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [std_add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L4)                                                 | 6           | 0           |
| [std_add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L11)                                               | 6           | 0           |
| [std_add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L18)                                               | 6           | 0           |
| [std_add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L25)                                               | 6           | 0           |
| [std_add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L32)                                              | 6           | 0           |
| [std_sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L4)                                                 | 6           | 0           |
| [std_sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L11)                                               | 6           | 0           |
| [std_sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L18)                                               | 6           | 0           |
| [std_sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L25)                                               | 6           | 0           |
| [std_sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L32)                                              | 6           | 0           |
| [std_mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L4)                                                 | 6           | 0           |
| [std_mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L11)                                               | 6           | 0           |
| [std_mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L18)                                               | 6           | 0           |
| [std_mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L25)                                               | 6           | 0           |
| [std_mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L32)                                              | 6           | 0           |
| [std_div_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L4)                                                 | 6           | 0           |
| [std_div_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L11)                                               | 6           | 0           |
| [std_div_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L18)                                               | 6           | 0           |
| [std_div_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L25)                                               | 6           | 0           |
| [std_div_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L32)                                              | 6           | 0           |

## Sub

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [sub_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L4)                                           | 7           | 1           |
| [sub_assign_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L12)                                         | 7           | 1           |
| [sub_assign_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L20)                                         | 7           | 1           |
| [sub_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L28)                                         | 7           | 1           |
| [sub_assign_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L36)                                        | 9           | 3           |

## Vec

| Function                                                                                                                                         | CU Consumed | CU Adjusted |
|--------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
| [vec_u8_new](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_new.rs#L4)                                                      | 9           | 3           |
| [vec_u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_with_capacity.rs#L4)                               | 113         | 107         |
| [vec_u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_with_capacity.rs#L9)                              | 113         | 107         |
| [vec_push_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L5)                                                    | 73          | 67          |
| [vec_push_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L12)                                                  | 78          | 72          |
| [vec_push_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L19)                                               | 85          | 79          |
| [vec_push_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L26)                                                | 202         | 196         |
| [vec_push_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L35)                                               | 313         | 307         |
| [vec_push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L44)                                            | 380         | 374         |
| [vec_push_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L53)                                     | 120         | 114         |
| [vec_push_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L60)                                    | 120         | 114         |
| [vec_push_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L67)                                 | 128         | 122         |
| [vec_push_10_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L74)                                  | 153         | 147         |
| [vec_push_10_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L83)                                 | 149         | 143         |
| [vec_push_10_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L92)                              | 218         | 212         |
