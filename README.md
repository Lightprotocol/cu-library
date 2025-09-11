# CU Library Benchmarks

Benchmark results for Solana runtime operations:

## Array

| Function                                    | CU Consumed | Heap Bytes Used |
|---------------------------------------------|-------------|-----------------|
| array_vec_new                               | 9           | 0               |
| array_vec_push_pubkey                       | 9           | 0               |
| array_vec_push_u64                          | 8           | 0               |
| array_vec_push_u8                           | 9           | 0               |
| array_vec_with_capacity_10                  | 9           | 0               |
| array_vec_with_capacity_100                 | 9           | 0               |

## Pinocchio

| Function                                    | CU Consumed | Heap Bytes Used |
|---------------------------------------------|-------------|-----------------|
| pinocchio_msg10_chars                       | 112         | 0               |
| pinocchio_sysvar_rent_exemption_165         | 153         | 0               |
| pinocchio_clock_get_slot                    | 174         | 0               |

## Solana

| Function                                    | CU Consumed | Heap Bytes Used |
|---------------------------------------------|-------------|-----------------|
| solana_msg10_chars                          | 112         | 0               |
| solana_msg_program_id                       | 6978        | 0               |
| solana_pubkey_new_from_array                | 9           | 0               |

