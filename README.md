# CU Library Benchmarks

Benchmark results for Solana runtime operations:

**Note:** The `#[profile]` macro adds ~5-6 CU overhead to each measurement.

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
| arrayvec_new                                | 7           |
| arrayvec_push_pubkey                        | 50          |
| arrayvec_push_u64                           | 43          |
| arrayvec_push_u8                            | 25          |
| arrayvec_with_capacity_10                   | 7           |
| arrayvec_with_capacity_100                  | 7           |
| arrayvec_push_10_u8                         | 10          |
| arrayvec_push_10_u64                        | 17          |
| arrayvec_push_10_pubkey                     | 87          |

## Baseline

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| baseline_empty                              | 6           |

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
| solana_msg_program_id                       | 6952        |
| solana_pubkey_new_from_array                | 14          |

## Vec

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| vec_new                                     | 9           |
| vec_with_capacity_10                        | 39          |
| vec_with_capacity_100                       | 39          |
| vec_push_u8                                 | 78          |
| vec_push_u64                                | 83          |
| vec_push_pubkey                             | 89          |
| vec_push_10_u8                              | 207         |
| vec_push_10_u64                             | 318         |
| vec_push_10_pubkey                          | 384         |

