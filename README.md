# CU Library Benchmarks

Benchmark results for Solana runtime operations:

**Note:** The `#[profile]` macro adds ~5-6 CU overhead to each measurement.

## Baseline

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| baseline_empty_function                               | 6           |

## Account

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| account_info_key                                      | 10          |
| account_info_owner                                    | 10          |
| account_info_is_signer                                | 13          |
| account_info_is_writable                              | 12          |
| account_info_executable                               | 13          |
| account_info_data_len                                 | 10          |
| account_info_lamports                                 | 10          |
| account_info_data_is_empty                            | 13          |
| account_info_is_owned_by                              | 31          |
| account_info_assign                                   | 17          |
| account_info_is_borrowed                              | 13          |
| account_info_borrow_lamports_unchecked                | 10          |
| account_info_borrow_mut_lamports_unchecked            | 10          |
| account_info_borrow_data_unchecked                    | 9           |
| account_info_borrow_mut_data_unchecked                | 9           |
| account_info_try_borrow_lamports                      | 16          |
| account_info_try_borrow_mut_lamports                  | 11          |
| account_info_can_borrow_lamports                      | 9           |
| account_info_can_borrow_mut_lamports                  | 9           |
| account_info_try_borrow_data                          | 15          |
| account_info_try_borrow_mut_data                      | 12          |
| account_info_can_borrow_data                          | 9           |
| account_info_can_borrow_mut_data                      | 9           |
| account_info_realloc                                  | 17          |

## Add

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| add_assign_u8                                         | 7           |
| add_assign_u16                                        | 7           |
| add_assign_u32                                        | 7           |
| add_assign_u64                                        | 7           |
| add_assign_u128                                       | 9           |

## Array

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| array_new                                             | 10          |
| array_with_capacity_10                                | 8           |
| array_with_capacity_100                               | 27          |
| array_assign_u8                                       | 9           |
| array_assign_u64                                      | 16          |
| array_assign_pubkey                                   | 108         |
| array_assign_10_u8                                    | 9           |
| array_assign_10_u64                                   | 16          |
| array_assign_10_pubkey                                | 108         |

## Arrayvec

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| arrayvec_u8_new                                       | 7           |
| arrayvec_push_pubkey                                  | 50          |
| arrayvec_push_u64                                     | 43          |
| arrayvec_push_u8                                      | 25          |
| arrayvec_u8_with_capacity_10                          | 7           |
| arrayvec_u8_with_capacity_100                         | 7           |
| arrayvec_push_10_u8                                   | 10          |
| arrayvec_push_10_u64                                  | 17          |
| arrayvec_push_10_pubkey                               | 88          |

## Checked

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| checked_add_u8                                        | 7           |
| checked_add_u16                                       | 7           |
| checked_add_u32                                       | 8           |
| checked_add_u64                                       | 8           |
| checked_add_u128                                      | 10          |
| checked_sub_u8                                        | 7           |
| checked_sub_u16                                       | 7           |
| checked_sub_u32                                       | 8           |
| checked_sub_u64                                       | 8           |
| checked_sub_u128                                      | 10          |
| checked_mul_u8                                        | 7           |
| checked_mul_u16                                       | 7           |
| checked_mul_u32                                       | 8           |
| checked_mul_u64                                       | 8           |
| checked_mul_u128                                      | 10          |
| checked_div_u8                                        | 7           |
| checked_div_u16                                       | 7           |
| checked_div_u32                                       | 8           |
| checked_div_u64                                       | 8           |
| checked_div_u128                                      | 10          |

## Conversions

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| conversions_try_into_slice_to_array_32_unwrap         | 15          |
| conversions_try_into_slice_to_array_32_map_err        | 6           |
| conversions_try_into_usize_to_u64_unwrap              | 7           |
| conversions_try_into_usize_to_u64_map_err             | 6           |
| conversions_try_into_u64_to_usize_unwrap              | 7           |
| conversions_try_into_u64_to_usize_map_err             | 6           |
| conversions_try_into_u32_to_usize_unwrap              | 7           |
| conversions_try_into_u32_to_usize_map_err             | 6           |
| conversions_try_into_u16_to_usize_unwrap              | 7           |
| conversions_try_into_u16_to_usize_map_err             | 6           |
| conversions_try_into_u8_to_usize_unwrap               | 7           |
| conversions_try_into_u8_to_usize_map_err              | 6           |
| conversions_u8_as_u16                                 | 7           |
| conversions_u8_as_u32                                 | 7           |
| conversions_u8_as_u64                                 | 7           |
| conversions_u8_as_usize                               | 7           |
| conversions_u16_as_u8                                 | 7           |
| conversions_u16_as_u32                                | 7           |
| conversions_u16_as_u64                                | 7           |
| conversions_u16_as_usize                              | 7           |
| conversions_u32_as_u8                                 | 7           |
| conversions_u32_as_u16                                | 7           |
| conversions_u32_as_u64                                | 7           |
| conversions_u32_as_usize                              | 7           |
| conversions_u64_as_u8                                 | 7           |
| conversions_u64_as_u16                                | 7           |
| conversions_u64_as_u32                                | 7           |
| conversions_u64_as_usize                              | 7           |

## Cpi

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| cpi_account_meta_array_10                             | 6           |
| cpi_account_info_array_10_ref                         | 6           |
| cpi_account_info_array_10_clone                       | 6           |
| cpi_account_info_array_10_move                        | 6           |
| cpi_arrayvec_push_account_meta_10                     | 182         |
| cpi_arrayvec_push_account_info_10_ref                 | 6           |
| cpi_arrayvec_push_account_info_10_clone               | 8           |
| cpi_arrayvec_push_account_info_10_move                | 8           |
| cpi_account_meta_array_10_loop                        | 201         |
| cpi_account_info_array_10_ref_loop                    | 6           |
| cpi_account_info_array_10_clone_loop                  | 8           |
| cpi_account_info_array_10_move_loop                   | 8           |

## Option

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| option_checked_add_u8_unwrap                          | 7           |
| option_checked_add_u8_ok_or                           | 6           |
| option_checked_add_u8_ok_or_else                      | 6           |
| option_checked_add_u8_unwrap_or_default               | 8           |
| option_checked_add_u8_unwrap_or                       | 8           |
| option_slice_get_array_unwrap                         | 15          |
| option_slice_get_array_ok_or                          | 6           |
| option_slice_get_array_ok_or_else                     | 6           |
| option_slice_get_array_unwrap_or_default              | 15          |
| option_slice_get_array_unwrap_or                      | 15          |
| option_pubkey_ref_map_deref                           | 6           |
| option_pubkey_as_ref_map_convert                      | 6           |
| option_if_let_some_u8                                 | 6           |
| option_if_let_some_array                              | 15          |
| option_if_let_some_pubkey                             | 15          |
| option_if_let_some_array_ref                          | 6           |

## Partial

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| partial_eq_u8                                         | 7           |
| partial_eq_u16                                        | 7           |
| partial_eq_u32                                        | 7           |
| partial_eq_u64                                        | 7           |
| partial_eq_u128                                       | 7           |
| partial_eq_array_u8_32_ref                            | 7           |
| partial_eq_array_u8_32                                | 6           |
| partial_eq_array_u16_32                               | 7           |
| partial_eq_array_u32_32                               | 7           |
| partial_eq_array_u64_32                               | 7           |
| partial_eq_u8_neq                                     | 9           |
| partial_eq_u16_neq                                    | 9           |
| partial_eq_u32_neq                                    | 10          |
| partial_eq_u64_neq                                    | 10          |
| partial_eq_u128_neq                                   | 13          |
| partial_eq_array_u8_32_neq_ref                        | 35          |
| partial_eq_array_u8_32_neq                            | 31          |
| partial_eq_array_u8_32_neq_deref                      | 35          |
| partial_eq_array_u16_32_neq                           | 33          |
| partial_eq_array_u32_32_neq                           | 34          |
| partial_eq_array_u64_32_neq                           | 34          |

## Pinocchio

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| pinocchio_msg10_chars                                 | 109         |
| pinocchio_sysvar_rent_exemption_165                   | 153         |
| pinocchio_clock_get_slot                              | 171         |

## Saturating

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| saturating_add_u8                                     | 7           |
| saturating_add_u16                                    | 7           |
| saturating_add_u32                                    | 7           |
| saturating_add_u64                                    | 7           |
| saturating_add_u128                                   | 8           |
| saturating_sub_u8                                     | 7           |
| saturating_sub_u16                                    | 7           |
| saturating_sub_u32                                    | 7           |
| saturating_sub_u64                                    | 7           |
| saturating_sub_u128                                   | 8           |
| saturating_mul_u8                                     | 7           |
| saturating_mul_u16                                    | 7           |
| saturating_mul_u32                                    | 7           |
| saturating_mul_u64                                    | 7           |
| saturating_mul_u128                                   | 8           |

## Solana

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| solana_msg10_chars                                    | 110         |
| solana_msg_program_id                                 | 6954        |
| solana_pubkey_new_from_array                          | 15          |

## Std

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| std_add_u8                                            | 6           |
| std_add_u16                                           | 6           |
| std_add_u32                                           | 6           |
| std_add_u64                                           | 6           |
| std_add_u128                                          | 6           |
| std_sub_u8                                            | 6           |
| std_sub_u16                                           | 6           |
| std_sub_u32                                           | 6           |
| std_sub_u64                                           | 6           |
| std_sub_u128                                          | 6           |
| std_mul_u8                                            | 6           |
| std_mul_u16                                           | 6           |
| std_mul_u32                                           | 6           |
| std_mul_u64                                           | 6           |
| std_mul_u128                                          | 6           |
| std_div_u8                                            | 6           |
| std_div_u16                                           | 6           |
| std_div_u32                                           | 6           |
| std_div_u64                                           | 6           |
| std_div_u128                                          | 6           |

## Sub

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| sub_assign_u8                                         | 7           |
| sub_assign_u16                                        | 7           |
| sub_assign_u32                                        | 7           |
| sub_assign_u64                                        | 7           |
| sub_assign_u128                                       | 9           |

## Vec

| Function                                              | CU Consumed |
|-------------------------------------------------------|-------------|
| vec_u8_new                                            | 9           |
| vec_u8_with_capacity_10                               | 113         |
| vec_u8_with_capacity_100                              | 113         |
| vec_push_u8                                           | 78          |
| vec_push_u64                                          | 83          |
| vec_push_pubkey                                       | 90          |
| vec_push_10_u8                                        | 207         |
| vec_push_10_u64                                       | 318         |
| vec_push_10_pubkey                                    | 385         |
| vec_push_u8_with_capacity                             | 120         |
| vec_push_u64_with_capacity                            | 120         |
| vec_push_pubkey_with_capacity                         | 128         |
| vec_push_10_u8_with_capacity                          | 153         |
| vec_push_10_u64_with_capacity                         | 149         |
| vec_push_10_pubkey_with_capacity                      | 218         |

