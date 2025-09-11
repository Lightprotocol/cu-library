# CU Library Benchmarks

Benchmark results for Solana runtime operations:

**Note:** The `#[profile]` macro adds ~5-6 CU overhead to each measurement.

## Baseline

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| baseline_empty_function                     | 6           |

## Account

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| account_info_key                            | 9           |
| account_info_owner                          | 9           |
| account_info_is_signer                      | 12          |
| account_info_is_writable                    | 11          |
| account_info_executable                     | 12          |
| account_info_data_len                       | 9           |
| account_info_lamports                       | 9           |
| account_info_data_is_empty                  | 12          |
| account_info_is_owned_by                    | 30          |
| account_info_assign                         | 15          |
| account_info_is_borrowed                    | 12          |
| account_info_borrow_lamports_unchecked      | 9           |
| account_info_borrow_mut_lamports_unchecked  | 9           |
| account_info_borrow_data_unchecked          | 8           |
| account_info_borrow_mut_data_unchecked      | 8           |
| account_info_try_borrow_lamports            | 15          |
| account_info_try_borrow_mut_lamports        | 10          |
| account_info_can_borrow_lamports            | 8           |
| account_info_can_borrow_mut_lamports        | 8           |
| account_info_try_borrow_data                | 14          |
| account_info_try_borrow_mut_data            | 11          |
| account_info_can_borrow_data                | 8           |
| account_info_can_borrow_mut_data            | 8           |
| account_info_realloc                        | 16          |
| account_info_close                          | 27          |
| account_info_close_unchecked                | 21          |

## Add

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| add_assign_u8                               | 7           |
| add_assign_u16                              | 7           |
| add_assign_u32                              | 7           |
| add_assign_u64                              | 7           |
| add_assign_u128                             | 9           |

## Array

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| array_new                                   | 10          |
| array_with_capacity_10                      | 8           |
| array_with_capacity_100                     | 27          |
| array_assign_u8                             | 9           |
| array_assign_u64                            | 16          |
| array_assign_pubkey                         | 107         |
| array_assign_10_u8                          | 9           |
| array_assign_10_u64                         | 16          |
| array_assign_10_pubkey                      | 107         |

## Arrayvec

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| arrayvec_u8_new                             | 7           |
| arrayvec_push_pubkey                        | 50          |
| arrayvec_push_u64                           | 43          |
| arrayvec_push_u8                            | 25          |
| arrayvec_u8_with_capacity_10                | 7           |
| arrayvec_u8_with_capacity_100               | 7           |
| arrayvec_push_10_u8                         | 10          |
| arrayvec_push_10_u64                        | 17          |
| arrayvec_push_10_pubkey                     | 87          |

## Checked

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| checked_add_u8                              | 7           |
| checked_add_u16                             | 7           |
| checked_add_u32                             | 8           |
| checked_add_u64                             | 8           |
| checked_add_u128                            | 10          |
| checked_sub_u8                              | 7           |
| checked_sub_u16                             | 7           |
| checked_sub_u32                             | 8           |
| checked_sub_u64                             | 8           |
| checked_sub_u128                            | 10          |
| checked_mul_u8                              | 7           |
| checked_mul_u16                             | 7           |
| checked_mul_u32                             | 8           |
| checked_mul_u64                             | 8           |
| checked_mul_u128                            | 10          |
| checked_div_u8                              | 7           |
| checked_div_u16                             | 7           |
| checked_div_u32                             | 8           |
| checked_div_u64                             | 8           |
| checked_div_u128                            | 10          |

## Pinocchio

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| pinocchio_msg10_chars                       | 110         |
| pinocchio_sysvar_rent_exemption_165         | 153         |
| pinocchio_clock_get_slot                    | 170         |

## Saturating

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| saturating_add_u8                           | 7           |
| saturating_add_u16                          | 7           |
| saturating_add_u32                          | 7           |
| saturating_add_u64                          | 7           |
| saturating_add_u128                         | 8           |
| saturating_sub_u8                           | 7           |
| saturating_sub_u16                          | 7           |
| saturating_sub_u32                          | 7           |
| saturating_sub_u64                          | 7           |
| saturating_sub_u128                         | 8           |
| saturating_mul_u8                           | 7           |
| saturating_mul_u16                          | 7           |
| saturating_mul_u32                          | 7           |
| saturating_mul_u64                          | 7           |
| saturating_mul_u128                         | 8           |

## Solana

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| solana_msg10_chars                          | 110         |
| solana_msg_program_id                       | 6954        |
| solana_pubkey_new_from_array                | 14          |

## Std

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| std_add_u8                                  | 6           |
| std_add_u16                                 | 6           |
| std_add_u32                                 | 6           |
| std_add_u64                                 | 6           |
| std_add_u128                                | 6           |
| std_sub_u8                                  | 6           |
| std_sub_u16                                 | 6           |
| std_sub_u32                                 | 6           |
| std_sub_u64                                 | 6           |
| std_sub_u128                                | 6           |
| std_mul_u8                                  | 6           |
| std_mul_u16                                 | 6           |
| std_mul_u32                                 | 6           |
| std_mul_u64                                 | 6           |
| std_mul_u128                                | 6           |
| std_div_u8                                  | 6           |
| std_div_u16                                 | 6           |
| std_div_u32                                 | 6           |
| std_div_u64                                 | 6           |
| std_div_u128                                | 6           |

## Sub

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| sub_assign_u8                               | 7           |
| sub_assign_u16                              | 7           |
| sub_assign_u32                              | 7           |
| sub_assign_u64                              | 7           |
| sub_assign_u128                             | 9           |

## Vec

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| vec_u8_new                                  | 9           |
| vec_u8_with_capacity_10                     | 113         |
| vec_u8_with_capacity_100                    | 113         |
| vec_push_u8                                 | 78          |
| vec_push_u64                                | 83          |
| vec_push_pubkey                             | 89          |
| vec_push_10_u8                              | 207         |
| vec_push_10_u64                             | 318         |
| vec_push_10_pubkey                          | 384         |
| vec_push_u8_with_capacity                   | 120         |
| vec_push_u64_with_capacity                  | 120         |
| vec_push_pubkey_with_capacity               | 127         |
| vec_push_10_u8_with_capacity                | 153         |
| vec_push_10_u64_with_capacity               | 149         |
| vec_push_10_pubkey_with_capacity            | 217         |

