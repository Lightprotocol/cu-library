# CU Library Benchmarks

Benchmark results for Solana runtime operations.

## Table of Contents

**[1. Baseline](#1-baseline)**

**[2. Account Info](#2-account-info)**

**[3. Checked Math](#3-checked-math)**

**[4. Collections](#4-collections)**

**[5. Conversions](#5-conversions)**

**[6. Cpi](#6-cpi)**

**[7. Option](#7-option)**

**[8. Partial Eq](#8-partial-eq)**

**[9. Pinocchio Ops](#9-pinocchio-ops)**

**[10. Saturating Math](#10-saturating-math)**

**[11. Serialization](#11-serialization)**

**[12. Solana Ops](#12-solana-ops)**

**[13. Std Math](#13-std-math)**


## Definitions

- **CU Consumed**: Total compute units consumed by the profiled function
- **CU Adjusted**: Actual operation cost with baseline profiling overhead subtracted (CU Consumed - Baseline CU)
- **Baseline CU**: CU consumed by an empty profiled function (`#[profile]` macro overhead)

## 1. Baseline

  ### 1.1 Lib

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [baseline_empty_function](https://github.com/Lightprotocol/cu-library/blob/main/src/lib.rs#L21)                                                                                      | 6           | N/A         |

## 2. Account Info

  ### 2.1 Account Borrows

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [is_borrowed](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L5)                                                                          | 12          | 6           |
  | [borrow_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L10)                                                           | 9           | 3           |
  | [borrow_mut_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L15)                                                       | 9           | 3           |
  | [borrow_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L21)                                                               | 8           | 2           |
  | [borrow_mut_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L26)                                                           | 8           | 2           |
  | [try_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L32)                                                                 | 16          | 10          |
  | [try_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L38)                                                             | 11          | 5           |
  | [can_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L44)                                                                 | 8           | 2           |
  | [can_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L49)                                                             | 8           | 2           |
  | [try_borrow_data](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L54)                                                                     | 15          | 9           |
  | [try_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L60)                                                                 | 12          | 6           |
  | [can_borrow_data](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L66)                                                                     | 8           | 2           |
  | [can_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_borrows.rs#L71)                                                                 | 8           | 2           |

  ### 2.2 Account Checks

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [is_signer](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_checks.rs#L5)                                                                             | 12          | 6           |
  | [is_writable](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_checks.rs#L10)                                                                          | 11          | 5           |
  | [executable](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_checks.rs#L15)                                                                           | 12          | 6           |
  | [data_is_empty](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_checks.rs#L20)                                                                        | 12          | 6           |

  ### 2.3 Account Data

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [data_len](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_data.rs#L5)                                                                                | 9           | 3           |
  | [lamports](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_data.rs#L10)                                                                               | 9           | 3           |

  ### 2.4 Account Key

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [key](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_key.rs#L5)                                                                                      | 9           | 3           |

  ### 2.5 Account Owner

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [owner](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_owner.rs#L5)                                                                                  | 9           | 3           |

  ### 2.6 Account Ownership

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [is_owned_by](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_ownership.rs#L5)                                                                        | 30          | 24          |
  | [assign](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_ownership.rs#L10)                                                                            | 15          | 9           |

  ### 2.7 Account Realloc

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [realloc](https://github.com/Lightprotocol/cu-library/blob/main/src/account_info/account_realloc.rs#L5)                                                                              | 16          | 10          |

## 3. Checked Math

  ### 3.1 Checked Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_add.rs#L4)                                                                                   | 7           | 1           |
  | [add_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_add.rs#L11)                                                                                 | 7           | 1           |
  | [add_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_add.rs#L18)                                                                                 | 8           | 2           |
  | [add_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_add.rs#L25)                                                                                 | 8           | 2           |
  | [add_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_add.rs#L32)                                                                                | 10          | 4           |

  ### 3.2 Checked Div

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [div_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_div.rs#L4)                                                                                   | 7           | 1           |
  | [div_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_div.rs#L11)                                                                                 | 7           | 1           |
  | [div_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_div.rs#L18)                                                                                 | 8           | 2           |
  | [div_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_div.rs#L25)                                                                                 | 8           | 2           |
  | [div_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_div.rs#L32)                                                                                | 10          | 4           |

  ### 3.3 Checked Mul

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [mul_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_mul.rs#L4)                                                                                   | 7           | 1           |
  | [mul_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_mul.rs#L11)                                                                                 | 7           | 1           |
  | [mul_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_mul.rs#L18)                                                                                 | 8           | 2           |
  | [mul_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_mul.rs#L25)                                                                                 | 8           | 2           |
  | [mul_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_mul.rs#L32)                                                                                | 10          | 4           |

  ### 3.4 Checked Sub

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_sub.rs#L4)                                                                                   | 7           | 1           |
  | [sub_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_sub.rs#L11)                                                                                 | 7           | 1           |
  | [sub_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_sub.rs#L18)                                                                                 | 8           | 2           |
  | [sub_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_sub.rs#L25)                                                                                 | 8           | 2           |
  | [sub_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/checked_math/checked_sub.rs#L32)                                                                                | 10          | 4           |

## 4. Collections

  ### 4.1 Array

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [new](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_new.rs#L4)                                                                                   | 10          | 4           |
  | [with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_with_capacity.rs#L4)                                                            | 8           | 2           |
  | [with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_with_capacity.rs#L9)                                                           | 27          | 21          |
  | [assign_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_assign.rs#L5)                                                                          | 9           | 3           |
  | [assign_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_assign.rs#L12)                                                                        | 16          | 10          |
  | [assign_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_assign.rs#L19)                                                                     | 108         | 102         |
  | [assign_10_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_assign.rs#L26)                                                                      | 9           | 3           |
  | [assign_10_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_assign.rs#L35)                                                                     | 16          | 10          |
  | [assign_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_assign.rs#L44)                                                                  | 108         | 102         |
  | [array_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u8_32.rs#L4)                                                                   | 9           | 3           |
  | [array_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u8_32.rs#L9)                                                                     | 6           | 0           |
  | [array_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u8_32.rs#L14)                                                              | 6           | 0           |
  | [array_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u8_32.rs#L19)                                                             | 9           | 3           |
  | [array_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u64_10.rs#L4)                                                                 | 9           | 3           |
  | [array_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u64_10.rs#L9)                                                                   | 6           | 0           |
  | [array_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u64_10.rs#L14)                                                            | 6           | 0           |
  | [array_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/array/array_u64_10.rs#L19)                                                           | 9           | 3           |

  ### 4.2 Arrayvec

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_new](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_new.rs#L5)                                                                               | 7           | 1           |
  | [push_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_push.rs#L29)                                                                        | 29          | 23          |
  | [push_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_push.rs#L24)                                                                           | 22          | 16          |
  | [push_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_push.rs#L19)                                                                            | 20          | 14          |
  | [u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_with_capacity.rs#L5)                                                        | 7           | 1           |
  | [u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_with_capacity.rs#L10)                                                      | 7           | 1           |
  | [push_10_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_push.rs#L34)                                                                         | 10          | 4           |
  | [push_10_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_push.rs#L41)                                                                        | 17          | 11          |
  | [push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/vec_push.rs#L48)                                                                     | 87          | 81          |
  | [get_first_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L37)                                                                     | 6           | 0           |
  | [get_10th_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L42)                                                                      | 6           | 0           |
  | [find_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L47)                                                                  | 18          | 12          |
  | [find_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L57)                                                                 | 38          | 32          |
  | [position_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L67)                                                              | 42          | 36          |
  | [position_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L72)                                                             | 96          | 90          |
  | [update_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L77)                                                                         | 14          | 8           |
  | [update_get_mut](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L82)                                                                       | 14          | 8           |
  | [update_iter_mut_find](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/arrayvec/access.rs#L93)                                                                 | 24          | 18          |

  ### 4.3 Heapless

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_new](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_new.rs#L5)                                                                               | 6           | 0           |
  | [u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_with_capacity.rs#L5)                                                        | 6           | 0           |
  | [u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_with_capacity.rs#L10)                                                      | 6           | 0           |
  | [push_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_push.rs#L19)                                                                            | 10          | 4           |
  | [push_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_push.rs#L24)                                                                           | 10          | 4           |
  | [push_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_push.rs#L29)                                                                        | 15          | 9           |
  | [push_10_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_push.rs#L34)                                                                         | 84          | 78          |
  | [push_10_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_push.rs#L41)                                                                        | 101         | 95          |
  | [push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/vec_push.rs#L48)                                                                     | 88          | 82          |
  | [get_first_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L37)                                                                     | 6           | 0           |
  | [get_10th_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L42)                                                                      | 6           | 0           |
  | [find_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L47)                                                                  | 18          | 12          |
  | [find_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L57)                                                                 | 38          | 32          |
  | [position_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L67)                                                              | 42          | 36          |
  | [position_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L72)                                                             | 96          | 90          |
  | [update_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L77)                                                                         | 14          | 8           |
  | [update_get_mut](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L82)                                                                       | 14          | 8           |
  | [update_iter_mut_find](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/heapless/access.rs#L93)                                                                 | 23          | 17          |

  ### 4.4 Smallvec

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_new](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_new.rs#L5)                                                                               | 6           | 0           |
  | [u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_with_capacity.rs#L5)                                                        | 6           | 0           |
  | [u8_with_capacity_128](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_with_capacity.rs#L10)                                                      | 6           | 0           |
  | [push_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_push.rs#L19)                                                                            | 25          | 19          |
  | [push_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_push.rs#L24)                                                                           | 27          | 21          |
  | [push_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_push.rs#L29)                                                                        | 33          | 27          |
  | [push_10_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_push.rs#L34)                                                                         | 10          | 4           |
  | [push_10_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_push.rs#L41)                                                                        | 17          | 11          |
  | [push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/vec_push.rs#L48)                                                                     | 87          | 81          |
  | [get_first_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L37)                                                                     | 6           | 0           |
  | [get_10th_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L42)                                                                      | 6           | 0           |
  | [find_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L47)                                                                  | 20          | 14          |
  | [find_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L57)                                                                 | 38          | 32          |
  | [position_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L67)                                                              | 44          | 38          |
  | [position_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L75)                                                             | 96          | 90          |
  | [update_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L83)                                                                         | 14          | 8           |
  | [update_get_mut](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L88)                                                                       | 14          | 8           |
  | [update_iter_mut_find](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/smallvec/access.rs#L99)                                                                 | 26          | 20          |

  ### 4.5 Tinyvec

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_new](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_new.rs#L5)                                                                                | 6           | 0           |
  | [u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_with_capacity.rs#L5)                                                         | 6           | 0           |
  | [u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_with_capacity.rs#L10)                                                       | 6           | 0           |
  | [push_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_push.rs#L19)                                                                             | 8           | 2           |
  | [push_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_push.rs#L24)                                                                            | 8           | 2           |
  | [push_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_push.rs#L29)                                                                         | 15          | 9           |
  | [push_10_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_push.rs#L34)                                                                          | 10          | 4           |
  | [push_10_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_push.rs#L41)                                                                         | 17          | 11          |
  | [push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/vec_push.rs#L48)                                                                      | 87          | 81          |
  | [get_first_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L37)                                                                      | 6           | 0           |
  | [get_10th_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L42)                                                                       | 6           | 0           |
  | [find_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L47)                                                                   | 18          | 12          |
  | [find_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L57)                                                                  | 38          | 32          |
  | [position_pubkey_1_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L67)                                                               | 42          | 36          |
  | [position_pubkey_10_iters](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L72)                                                              | 96          | 90          |
  | [update_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L77)                                                                          | 14          | 8           |
  | [update_get_mut](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L82)                                                                        | 14          | 8           |
  | [update_iter_mut_find](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/tinyvec/access.rs#L93)                                                                  | 24          | 18          |

  ### 4.6 Vec

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_new](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_new.rs#L4)                                                                                    | 9           | 3           |
  | [u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_with_capacity.rs#L4)                                                             | 113         | 107         |
  | [u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_with_capacity.rs#L9)                                                            | 113         | 107         |
  | [push_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L5)                                                                                  | 73          | 67          |
  | [push_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L12)                                                                                | 78          | 72          |
  | [push_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L19)                                                                             | 84          | 78          |
  | [push_10_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L26)                                                                              | 202         | 196         |
  | [push_10_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L35)                                                                             | 313         | 307         |
  | [push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L44)                                                                          | 382         | 376         |
  | [push_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L53)                                                                   | 120         | 114         |
  | [push_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L60)                                                                  | 120         | 114         |
  | [push_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L67)                                                               | 127         | 121         |
  | [push_10_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L74)                                                                | 153         | 147         |
  | [push_10_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L83)                                                               | 149         | 143         |
  | [push_10_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_push.rs#L92)                                                            | 218         | 212         |
  | [vec_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u8_32.rs#L4)                                                                         | 9           | 3           |
  | [vec_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u8_32.rs#L9)                                                                           | 8           | 2           |
  | [vec_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u8_32.rs#L14)                                                                    | 8           | 2           |
  | [vec_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u8_32.rs#L19)                                                                   | 9           | 3           |
  | [vec_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u64_10.rs#L4)                                                                       | 9           | 3           |
  | [vec_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u64_10.rs#L9)                                                                         | 8           | 2           |
  | [vec_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u64_10.rs#L14)                                                                  | 8           | 2           |
  | [vec_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/main/src/collections/vec/vec_u64_10.rs#L19)                                                                 | 9           | 3           |

## 5. Conversions

  ### 5.1 Cast U16

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u16_as_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u16.rs#L5)                                                                                    | 7           | 1           |
  | [u16_as_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u16.rs#L10)                                                                                  | 7           | 1           |
  | [u16_as_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u16.rs#L15)                                                                                  | 7           | 1           |
  | [u16_as_usize](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u16.rs#L20)                                                                                | 7           | 1           |

  ### 5.2 Cast U32

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u32_as_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u32.rs#L5)                                                                                    | 7           | 1           |
  | [u32_as_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u32.rs#L10)                                                                                  | 7           | 1           |
  | [u32_as_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u32.rs#L15)                                                                                  | 7           | 1           |
  | [u32_as_usize](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u32.rs#L20)                                                                                | 7           | 1           |

  ### 5.3 Cast U64

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u64_as_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u64.rs#L5)                                                                                    | 7           | 1           |
  | [u64_as_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u64.rs#L10)                                                                                  | 7           | 1           |
  | [u64_as_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u64.rs#L15)                                                                                  | 7           | 1           |
  | [u64_as_usize](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u64.rs#L20)                                                                                | 7           | 1           |

  ### 5.4 Cast U8

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_as_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u8.rs#L5)                                                                                     | 7           | 1           |
  | [u8_as_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u8.rs#L10)                                                                                    | 7           | 1           |
  | [u8_as_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u8.rs#L15)                                                                                    | 7           | 1           |
  | [u8_as_usize](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/cast_u8.rs#L20)                                                                                  | 7           | 1           |

  ### 5.5 From Usize

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [try_into_usize_to_u64_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/from_usize.rs#L6)                                                               | 7           | 1           |
  | [try_into_usize_to_u64_map_err](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/from_usize.rs#L11)                                                             | 6           | 0           |

  ### 5.6 Slice To Array

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [try_into_slice_to_array_32_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/slice_to_array.rs#L7)                                                      | 14          | 8           |
  | [try_into_slice_to_array_32_map_err](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/slice_to_array.rs#L12)                                                    | 6           | 0           |

  ### 5.7 To Usize

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [try_into_u64_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L6)                                                                 | 7           | 1           |
  | [try_into_u64_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L11)                                                               | 6           | 0           |
  | [try_into_u32_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L17)                                                                | 7           | 1           |
  | [try_into_u32_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L22)                                                               | 6           | 0           |
  | [try_into_u16_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L28)                                                                | 7           | 1           |
  | [try_into_u16_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L33)                                                               | 6           | 0           |
  | [try_into_u8_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L39)                                                                 | 7           | 1           |
  | [try_into_u8_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/main/src/conversions/to_usize.rs#L44)                                                                | 6           | 0           |

## 6. Cpi

  ### 6.1 Cpi Array Loop

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [account_meta_array_10_loop](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_array_loop.rs#L5)                                                                     | 201         | 195         |
  | [account_info_array_10_ref_loop](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_array_loop.rs#L28)                                                                | 6           | 0           |
  | [account_info_array_10_clone_loop](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_array_loop.rs#L41)                                                              | 7           | 1           |
  | [account_info_array_10_move_loop](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_array_loop.rs#L54)                                                               | 7           | 1           |

  ### 6.2 Cpi Arrays

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [account_meta_array_10](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrays.rs#L5)                                                                              | 6           | 0           |
  | [account_info_array_10_ref](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrays.rs#L61)                                                                         | 6           | 0           |
  | [account_info_array_10_clone](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrays.rs#L77)                                                                       | 6           | 0           |
  | [account_info_array_10_move](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrays.rs#L93)                                                                        | 6           | 0           |

  ### 6.3 Cpi Arrayvec

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [arrayvec_push_account_meta_10](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrayvec.rs#L6)                                                                    | 179         | 173         |
  | [arrayvec_push_account_info_10_ref](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrayvec.rs#L19)                                                               | 6           | 0           |
  | [arrayvec_push_account_info_10_clone](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrayvec.rs#L30)                                                             | 7           | 1           |
  | [arrayvec_push_account_info_10_move](https://github.com/Lightprotocol/cu-library/blob/main/src/cpi/cpi_arrayvec.rs#L41)                                                              | 7           | 1           |

## 7. Option

  ### 7.1 Option Checked Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [checked_add_u8_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_checked_add.rs#L7)                                                                   | 7           | 1           |
  | [checked_add_u8_ok_or](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_checked_add.rs#L13)                                                                   | 6           | 0           |
  | [checked_add_u8_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_checked_add.rs#L19)                                                              | 6           | 0           |
  | [checked_add_u8_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_checked_add.rs#L25)                                                       | 8           | 2           |
  | [checked_add_u8_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_checked_add.rs#L31)                                                               | 8           | 2           |

  ### 7.2 Option If Let

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [if_let_some_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_if_let.rs#L7)                                                                               | 6           | 0           |
  | [if_let_some_array](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_if_let.rs#L16)                                                                           | 14          | 8           |
  | [if_let_some_pubkey](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_if_let.rs#L25)                                                                          | 14          | 8           |
  | [if_let_some_array_ref](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_if_let.rs#L34)                                                                       | 6           | 0           |

  ### 7.3 Option Pubkey Ref

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [pubkey_ref_map_deref](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_pubkey_ref.rs#L7)                                                                     | 6           | 0           |
  | [pubkey_as_ref_map_convert](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_pubkey_ref.rs#L12)                                                               | 6           | 0           |

  ### 7.4 Option Slice Get

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [slice_get_array_unwrap](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_slice_get.rs#L7)                                                                    | 14          | 8           |
  | [slice_get_array_ok_or](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_slice_get.rs#L13)                                                                    | 6           | 0           |
  | [slice_get_array_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_slice_get.rs#L19)                                                               | 6           | 0           |
  | [slice_get_array_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_slice_get.rs#L25)                                                        | 14          | 8           |
  | [slice_get_array_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/main/src/option/option_slice_get.rs#L31)                                                                | 14          | 8           |

## 8. Partial Eq

  ### 8.1 Partial Eq Arrays

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [array_u8_32_ref](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_arrays.rs#L5)                                                                      | 7           | 1           |
  | [array_u8_32](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_arrays.rs#L11)                                                                         | 6           | 0           |
  | [array_u16_32](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_arrays.rs#L16)                                                                        | 7           | 1           |
  | [array_u32_32](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_arrays.rs#L21)                                                                        | 7           | 1           |
  | [array_u64_32](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_arrays.rs#L26)                                                                        | 7           | 1           |

  ### 8.2 Partial Eq Neq

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L5)                                                                                  | 9           | 3           |
  | [u16_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L10)                                                                                | 9           | 3           |
  | [u32_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L15)                                                                                | 10          | 4           |
  | [u64_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L20)                                                                                | 10          | 4           |
  | [u128_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L25)                                                                               | 13          | 7           |
  | [array_u8_32_neq_ref](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L32)                                                                    | 35          | 29          |
  | [array_u8_32_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L38)                                                                        | 31          | 25          |
  | [array_u8_32_neq_deref](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L44)                                                                  | 35          | 29          |
  | [array_u16_32_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L49)                                                                       | 33          | 27          |
  | [array_u32_32_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L54)                                                                       | 34          | 28          |
  | [array_u64_32_neq](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_neq.rs#L59)                                                                       | 34          | 28          |

  ### 8.3 Partial Eq Primitives

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_primitives.rs#L4)                                                                               | 7           | 1           |
  | [u16](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_primitives.rs#L9)                                                                              | 7           | 1           |
  | [u32](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_primitives.rs#L14)                                                                             | 7           | 1           |
  | [u64](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_primitives.rs#L19)                                                                             | 7           | 1           |
  | [u128](https://github.com/Lightprotocol/cu-library/blob/main/src/partial_eq/partial_eq_primitives.rs#L24)                                                                            | 7           | 1           |

## 9. Pinocchio Ops

  ### 9.1 Msg

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [msg10_chars](https://github.com/Lightprotocol/cu-library/blob/main/src/pinocchio_ops/msg.rs#L5)                                                                                     | 110         | 104         |

  ### 9.2 Sysvar Clock

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [clock_get_slot](https://github.com/Lightprotocol/cu-library/blob/main/src/pinocchio_ops/sysvar_clock.rs#L5)                                                                         | 171         | 165         |

  ### 9.3 Sysvar Rent

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sysvar_rent_exemption_165](https://github.com/Lightprotocol/cu-library/blob/main/src/pinocchio_ops/sysvar_rent.rs#L5)                                                               | 151         | 145         |

## 10. Saturating Math

  ### 10.1 Saturating Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_add.rs#L4)                                                                             | 7           | 1           |
  | [add_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_add.rs#L11)                                                                           | 7           | 1           |
  | [add_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_add.rs#L18)                                                                           | 7           | 1           |
  | [add_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_add.rs#L25)                                                                           | 7           | 1           |
  | [add_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_add.rs#L32)                                                                          | 8           | 2           |

  ### 10.2 Saturating Mul

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [mul_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_mul.rs#L4)                                                                             | 7           | 1           |
  | [mul_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_mul.rs#L11)                                                                           | 7           | 1           |
  | [mul_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_mul.rs#L18)                                                                           | 7           | 1           |
  | [mul_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_mul.rs#L25)                                                                           | 7           | 1           |
  | [mul_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_mul.rs#L32)                                                                          | 8           | 2           |

  ### 10.3 Saturating Sub

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_sub.rs#L4)                                                                             | 7           | 1           |
  | [sub_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_sub.rs#L11)                                                                           | 7           | 1           |
  | [sub_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_sub.rs#L18)                                                                           | 7           | 1           |
  | [sub_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_sub.rs#L25)                                                                           | 7           | 1           |
  | [sub_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/saturating_math/saturating_sub.rs#L32)                                                                          | 8           | 2           |

## 11. Serialization

  ### 11.1 Compressed Account Info

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [borsh_deserialize](https://github.com/Lightprotocol/cu-library/blob/main/src/serialization/compressed_account_info.rs#L200)                                                         | 428         | 422         |
  | [zero_copy_deserialize](https://github.com/Lightprotocol/cu-library/blob/main/src/serialization/compressed_account_info.rs#L206)                                                     | 130         | 124         |
  | [wincode_deserialize](https://github.com/Lightprotocol/cu-library/blob/main/src/serialization/compressed_account_info.rs#L215)                                                       | 594         | 588         |
  | [bincode_deserialize](https://github.com/Lightprotocol/cu-library/blob/main/src/serialization/compressed_account_info.rs#L220)                                                       | 3231        | 3225        |
  | [borsh1_deserialize](https://github.com/Lightprotocol/cu-library/blob/main/src/serialization/compressed_account_info.rs#L225)                                                        | 428         | 422         |
  | [rkyv_zero_copy_deserialize](https://github.com/Lightprotocol/cu-library/blob/main/src/serialization/compressed_account_info.rs#L230)                                                | 178         | 172         |
  | [wincode_shortvec_deserialize](https://github.com/Lightprotocol/cu-library/blob/main/src/serialization/compressed_account_info.rs#L285)                                              | 612         | 606         |

## 12. Solana Ops

  ### 12.1 Msg

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [msg10_chars](https://github.com/Lightprotocol/cu-library/blob/main/src/solana_ops/msg.rs#L5)                                                                                        | 110         | 104         |

  ### 12.2 Msg Program Id

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [msg_program_id](https://github.com/Lightprotocol/cu-library/blob/main/src/solana_ops/msg_program_id.rs#L5)                                                                          | 6953        | 6947        |

  ### 12.3 Pubkey New From Array

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [pubkey_new_from_array](https://github.com/Lightprotocol/cu-library/blob/main/src/solana_ops/pubkey_new_from_array.rs#L5)                                                            | 14          | 8           |

  ### 12.4 Pubkey To Bytes

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [pubkey_to_bytes](https://github.com/Lightprotocol/cu-library/blob/main/src/solana_ops/pubkey_to_bytes.rs#L6)                                                                        | 14          | 8           |

## 13. Std Math

  ### 13.1 Add Assign

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_assign_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/add_assign.rs#L4)                                                                                 | 7           | 1           |
  | [add_assign_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/add_assign.rs#L12)                                                                               | 7           | 1           |
  | [add_assign_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/add_assign.rs#L20)                                                                               | 7           | 1           |
  | [add_assign_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/add_assign.rs#L28)                                                                               | 7           | 1           |
  | [add_assign_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/add_assign.rs#L36)                                                                              | 9           | 3           |

  ### 13.2 Std Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_add.rs#L4)                                                                                           | 6           | 0           |
  | [add_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_add.rs#L11)                                                                                         | 6           | 0           |
  | [add_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_add.rs#L18)                                                                                         | 6           | 0           |
  | [add_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_add.rs#L25)                                                                                         | 6           | 0           |
  | [add_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_add.rs#L32)                                                                                        | 6           | 0           |

  ### 13.3 Std Div

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [div_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_div.rs#L4)                                                                                           | 6           | 0           |
  | [div_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_div.rs#L11)                                                                                         | 6           | 0           |
  | [div_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_div.rs#L18)                                                                                         | 6           | 0           |
  | [div_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_div.rs#L25)                                                                                         | 6           | 0           |
  | [div_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_div.rs#L32)                                                                                        | 6           | 0           |

  ### 13.4 Std Mul

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [mul_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_mul.rs#L4)                                                                                           | 6           | 0           |
  | [mul_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_mul.rs#L11)                                                                                         | 6           | 0           |
  | [mul_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_mul.rs#L18)                                                                                         | 6           | 0           |
  | [mul_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_mul.rs#L25)                                                                                         | 6           | 0           |
  | [mul_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_mul.rs#L32)                                                                                        | 6           | 0           |

  ### 13.5 Std Sub

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_sub.rs#L4)                                                                                           | 6           | 0           |
  | [sub_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_sub.rs#L11)                                                                                         | 6           | 0           |
  | [sub_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_sub.rs#L18)                                                                                         | 6           | 0           |
  | [sub_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_sub.rs#L25)                                                                                         | 6           | 0           |
  | [sub_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/std_sub.rs#L32)                                                                                        | 6           | 0           |

  ### 13.6 Sub Assign

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_assign_u8](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/sub_assign.rs#L4)                                                                                 | 7           | 1           |
  | [sub_assign_u16](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/sub_assign.rs#L12)                                                                               | 7           | 1           |
  | [sub_assign_u32](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/sub_assign.rs#L20)                                                                               | 7           | 1           |
  | [sub_assign_u64](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/sub_assign.rs#L28)                                                                               | 7           | 1           |
  | [sub_assign_u128](https://github.com/Lightprotocol/cu-library/blob/main/src/std_math/sub_assign.rs#L36)                                                                              | 9           | 3           |

