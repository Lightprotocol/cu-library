# CU Library Benchmarks

Benchmark results for Solana runtime operations:

## Array

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| array_vec_new                               | 7           |
| array_vec_push_pubkey                       | 15          |
| array_vec_push_u64                          | 8           |
| array_vec_push_u8                           | 8           |
| array_vec_with_capacity_10                  | 7           |
| array_vec_with_capacity_100                 | 7           |
| array_vec_push_10_u8                        | 10          |
| array_vec_push_10_u64                       | 17          |
| array_vec_push_10_pubkey                    | 87          |

## Pinocchio

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| pinocchio_msg10_chars                       | 110         |
| pinocchio_sysvar_rent_exemption_165         | 152         |
| pinocchio_clock_get_slot                    | 171         |

## Solana

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| solana_msg10_chars                          | 110         |
| solana_msg_program_id                       | 6941        |
| solana_pubkey_new_from_array                | 14          |

## Vec

| Function                                    | CU Consumed |
|---------------------------------------------|-------------|
| vec_new                                     | 9           |
| vec_with_capacity_10                        | 18          |
| vec_with_capacity_100                       | 18          |
| vec_push_u8                                 | 78          |
| vec_push_u64                                | 83          |
| vec_push_pubkey                             | 89          |
| vec_push_10_u8                              | 208         |
| vec_push_10_u64                             | 319         |
| vec_push_10_pubkey                          | 385         |

