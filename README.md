# CU Library Benchmarks

Benchmark results for Solana runtime operations.

## Table of Contents

**[1. Baseline](#1-baseline)**

**[2. Access](#2-access)**

**[3. Account Info](#3-account-info)**

**[4. Array](#4-array)**

**[5. Arrayvec](#5-arrayvec)**

**[6. Checked Math](#6-checked-math)**

**[7. Conversions](#7-conversions)**

**[8. Cpi](#8-cpi)**

**[9. Option](#9-option)**

**[10. Partial Eq](#10-partial-eq)**

**[11. Pinocchio Ops](#11-pinocchio-ops)**

**[12. Saturating Math](#12-saturating-math)**

**[13. Serialization](#13-serialization)**

**[14. Solana Ops](#14-solana-ops)**

**[15. Std Math](#15-std-math)**

**[16. Vec](#16-vec)**


## Definitions

- **CU Consumed**: Total compute units consumed by the profiled function
- **CU Adjusted**: Actual operation cost with baseline profiling overhead subtracted (CU Consumed - Baseline CU)
- **Baseline CU**: CU consumed by an empty profiled function (`#[profile]` macro overhead)

## 1. Baseline

  ### 1.1 Lib

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [baseline_empty_function](https://github.com/Lightprotocol/cu-library/blob/master/src/lib.rs#L104)                                                                                   | 6           | N/A         |

## 2. Access

  ### 2.1 Array U64 10

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [array_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L4)                                                                          | 9           | 3           |
  | [array_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L9)                                                                            | 6           | 0           |
  | [array_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L14)                                                                     | 6           | 0           |
  | [array_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u64_10.rs#L19)                                                                    | 9           | 3           |

  ### 2.2 Array U8 32

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [array_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L4)                                                                            | 9           | 3           |
  | [array_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L9)                                                                              | 6           | 0           |
  | [array_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L14)                                                                       | 6           | 0           |
  | [array_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/array_u8_32.rs#L19)                                                                      | 9           | 3           |

  ### 2.3 Vec U64 10

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [vec_u64_10_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L4)                                                                              | 9           | 3           |
  | [vec_u64_10_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L9)                                                                                | 8           | 2           |
  | [vec_u64_10_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L14)                                                                         | 8           | 2           |
  | [vec_u64_10_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u64_10.rs#L19)                                                                        | 9           | 3           |

  ### 2.4 Vec U8 32

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [vec_u8_32_index](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L4)                                                                                | 9           | 3           |
  | [vec_u8_32_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L9)                                                                                  | 8           | 2           |
  | [vec_u8_32_get_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L14)                                                                           | 8           | 2           |
  | [vec_u8_32_if_let_get](https://github.com/Lightprotocol/cu-library/blob/master/src/access/vec_u8_32.rs#L19)                                                                          | 9           | 3           |

## 3. Account Info

  ### 3.1 Account Borrows

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [is_borrowed](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L5)                                                                        | 13          | 7           |
  | [borrow_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L10)                                                         | 10          | 4           |
  | [borrow_mut_lamports_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L15)                                                     | 10          | 4           |
  | [borrow_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L20)                                                             | 9           | 3           |
  | [borrow_mut_data_unchecked](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L25)                                                         | 9           | 3           |
  | [try_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L30)                                                               | 16          | 10          |
  | [try_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L36)                                                           | 11          | 5           |
  | [can_borrow_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L42)                                                               | 9           | 3           |
  | [can_borrow_mut_lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L47)                                                           | 9           | 3           |
  | [try_borrow_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L52)                                                                   | 15          | 9           |
  | [try_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L58)                                                               | 12          | 6           |
  | [can_borrow_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L64)                                                                   | 9           | 3           |
  | [can_borrow_mut_data](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_borrows.rs#L69)                                                               | 9           | 3           |

  ### 3.2 Account Checks

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [is_signer](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L5)                                                                           | 13          | 7           |
  | [is_writable](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L10)                                                                        | 12          | 6           |
  | [executable](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L15)                                                                         | 13          | 7           |
  | [data_is_empty](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_checks.rs#L20)                                                                      | 13          | 7           |

  ### 3.3 Account Data

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [data_len](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_data.rs#L5)                                                                              | 10          | 4           |
  | [lamports](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_data.rs#L10)                                                                             | 10          | 4           |

  ### 3.4 Account Key

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [key](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_key.rs#L5)                                                                                    | 10          | 4           |

  ### 3.5 Account Owner

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [owner](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_owner.rs#L5)                                                                                | 10          | 4           |

  ### 3.6 Account Ownership

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [is_owned_by](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_ownership.rs#L5)                                                                      | 31          | 25          |
  | [assign](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_ownership.rs#L10)                                                                          | 17          | 11          |

  ### 3.7 Account Realloc

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [realloc](https://github.com/Lightprotocol/cu-library/blob/master/src/account_info/account_realloc.rs#L5)                                                                            | 17          | 11          |

## 4. Array

  ### 4.1 Array Assign

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L5)                                                                                    | 9           | 3           |
  | [assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L12)                                                                                  | 16          | 10          |
  | [assign_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L19)                                                                               | 108         | 102         |
  | [assign_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L26)                                                                                | 9           | 3           |
  | [assign_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L35)                                                                               | 16          | 10          |
  | [assign_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_assign.rs#L44)                                                                            | 108         | 102         |

  ### 4.2 Array New

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [new](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_new.rs#L4)                                                                                             | 10          | 4           |

  ### 4.3 Array With Capacity

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_with_capacity.rs#L4)                                                                      | 8           | 2           |
  | [with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/array/array_with_capacity.rs#L9)                                                                     | 27          | 21          |

## 5. Arrayvec

  ### 5.1 Vec New

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_new](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_new.rs#L5)                                                                                         | 7           | 1           |

  ### 5.2 Vec Push

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [push_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L20)                                                                                  | 50          | 44          |
  | [push_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L13)                                                                                     | 43          | 37          |
  | [push_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L6)                                                                                       | 25          | 19          |
  | [push_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L27)                                                                                   | 10          | 4           |
  | [push_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L36)                                                                                  | 17          | 11          |
  | [push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_push.rs#L45)                                                                               | 88          | 82          |

  ### 5.3 Vec With Capacity

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_with_capacity.rs#L5)                                                                  | 7           | 1           |
  | [u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/arrayvec/vec_with_capacity.rs#L10)                                                                | 7           | 1           |

## 6. Checked Math

  ### 6.1 Checked Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L4)                                                                                 | 7           | 1           |
  | [add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L11)                                                                               | 7           | 1           |
  | [add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L18)                                                                               | 8           | 2           |
  | [add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L25)                                                                               | 8           | 2           |
  | [add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L32)                                                                              | 10          | 4           |

  ### 6.2 Checked Div

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [div_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L4)                                                                                 | 7           | 1           |
  | [div_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L11)                                                                               | 7           | 1           |
  | [div_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L18)                                                                               | 8           | 2           |
  | [div_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L25)                                                                               | 8           | 2           |
  | [div_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_div.rs#L32)                                                                              | 10          | 4           |

  ### 6.3 Checked Mul

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L4)                                                                                 | 7           | 1           |
  | [mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L11)                                                                               | 7           | 1           |
  | [mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L18)                                                                               | 8           | 2           |
  | [mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L25)                                                                               | 8           | 2           |
  | [mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_mul.rs#L32)                                                                              | 10          | 4           |

  ### 6.4 Checked Sub

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L4)                                                                                 | 7           | 1           |
  | [sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L11)                                                                               | 7           | 1           |
  | [sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L18)                                                                               | 8           | 2           |
  | [sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L25)                                                                               | 8           | 2           |
  | [sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_sub.rs#L32)                                                                              | 10          | 4           |

## 7. Conversions

  ### 7.1 Cast U16

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u16_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L5)                                                                                  | 7           | 1           |
  | [u16_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L10)                                                                                | 7           | 1           |
  | [u16_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L15)                                                                                | 7           | 1           |
  | [u16_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u16.rs#L20)                                                                              | 7           | 1           |

  ### 7.2 Cast U32

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u32_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L5)                                                                                  | 7           | 1           |
  | [u32_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L10)                                                                                | 7           | 1           |
  | [u32_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L15)                                                                                | 7           | 1           |
  | [u32_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u32.rs#L20)                                                                              | 7           | 1           |

  ### 7.3 Cast U64

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u64_as_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L5)                                                                                  | 7           | 1           |
  | [u64_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L10)                                                                                | 7           | 1           |
  | [u64_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L15)                                                                                | 7           | 1           |
  | [u64_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u64.rs#L20)                                                                              | 7           | 1           |

  ### 7.4 Cast U8

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_as_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L5)                                                                                   | 7           | 1           |
  | [u8_as_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L10)                                                                                  | 7           | 1           |
  | [u8_as_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L15)                                                                                  | 7           | 1           |
  | [u8_as_usize](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/cast_u8.rs#L20)                                                                                | 7           | 1           |

  ### 7.5 From Usize

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [try_into_usize_to_u64_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/from_usize.rs#L6)                                                             | 7           | 1           |
  | [try_into_usize_to_u64_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/from_usize.rs#L11)                                                           | 6           | 0           |

  ### 7.6 Slice To Array

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [try_into_slice_to_array_32_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/slice_to_array.rs#L7)                                                    | 15          | 9           |
  | [try_into_slice_to_array_32_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/slice_to_array.rs#L12)                                                  | 6           | 0           |

  ### 7.7 To Usize

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [try_into_u64_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L6)                                                               | 7           | 1           |
  | [try_into_u64_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L11)                                                             | 6           | 0           |
  | [try_into_u32_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L17)                                                              | 7           | 1           |
  | [try_into_u32_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L22)                                                             | 6           | 0           |
  | [try_into_u16_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L28)                                                              | 7           | 1           |
  | [try_into_u16_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L33)                                                             | 6           | 0           |
  | [try_into_u8_to_usize_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L39)                                                               | 7           | 1           |
  | [try_into_u8_to_usize_map_err](https://github.com/Lightprotocol/cu-library/blob/master/src/conversions/to_usize.rs#L44)                                                              | 6           | 0           |

## 8. Cpi

  ### 8.1 Cpi Array Loop

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [account_meta_array_10_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L5)                                                                   | 201         | 195         |
  | [account_info_array_10_ref_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L28)                                                              | 6           | 0           |
  | [account_info_array_10_clone_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L41)                                                            | 8           | 2           |
  | [account_info_array_10_move_loop](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_array_loop.rs#L54)                                                             | 8           | 2           |

  ### 8.2 Cpi Arrays

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [account_meta_array_10](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L5)                                                                            | 6           | 0           |
  | [account_info_array_10_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L61)                                                                       | 6           | 0           |
  | [account_info_array_10_clone](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L77)                                                                     | 6           | 0           |
  | [account_info_array_10_move](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrays.rs#L93)                                                                      | 6           | 0           |

  ### 8.3 Cpi Arrayvec

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [arrayvec_push_account_meta_10](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L6)                                                                  | 182         | 176         |
  | [arrayvec_push_account_info_10_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L19)                                                             | 6           | 0           |
  | [arrayvec_push_account_info_10_clone](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L30)                                                           | 8           | 2           |
  | [arrayvec_push_account_info_10_move](https://github.com/Lightprotocol/cu-library/blob/master/src/cpi/cpi_arrayvec.rs#L41)                                                            | 8           | 2           |

## 9. Option

  ### 9.1 Option Checked Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [checked_add_u8_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L7)                                                                 | 7           | 1           |
  | [checked_add_u8_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L13)                                                                 | 6           | 0           |
  | [checked_add_u8_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L19)                                                            | 6           | 0           |
  | [checked_add_u8_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L25)                                                     | 8           | 2           |
  | [checked_add_u8_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_checked_add.rs#L31)                                                             | 8           | 2           |

  ### 9.2 Option If Let

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [if_let_some_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L7)                                                                             | 6           | 0           |
  | [if_let_some_array](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L16)                                                                         | 15          | 9           |
  | [if_let_some_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L25)                                                                        | 15          | 9           |
  | [if_let_some_array_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_if_let.rs#L34)                                                                     | 6           | 0           |

  ### 9.3 Option Pubkey Ref

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [pubkey_ref_map_deref](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_pubkey_ref.rs#L7)                                                                   | 6           | 0           |
  | [pubkey_as_ref_map_convert](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_pubkey_ref.rs#L12)                                                             | 6           | 0           |

  ### 9.4 Option Slice Get

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [slice_get_array_unwrap](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L7)                                                                  | 15          | 9           |
  | [slice_get_array_ok_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L13)                                                                  | 6           | 0           |
  | [slice_get_array_ok_or_else](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L19)                                                             | 6           | 0           |
  | [slice_get_array_unwrap_or_default](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L25)                                                      | 15          | 9           |
  | [slice_get_array_unwrap_or](https://github.com/Lightprotocol/cu-library/blob/master/src/option/option_slice_get.rs#L31)                                                              | 15          | 9           |

## 10. Partial Eq

  ### 10.1 Partial Eq Arrays

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [array_u8_32_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L5)                                                                    | 7           | 1           |
  | [array_u8_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L11)                                                                       | 6           | 0           |
  | [array_u16_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L16)                                                                      | 7           | 1           |
  | [array_u32_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L21)                                                                      | 7           | 1           |
  | [array_u64_32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_arrays.rs#L26)                                                                      | 7           | 1           |

  ### 10.2 Partial Eq Neq

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L5)                                                                                | 9           | 3           |
  | [u16_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L10)                                                                              | 9           | 3           |
  | [u32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L15)                                                                              | 10          | 4           |
  | [u64_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L20)                                                                              | 10          | 4           |
  | [u128_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L25)                                                                             | 13          | 7           |
  | [array_u8_32_neq_ref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L32)                                                                  | 35          | 29          |
  | [array_u8_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L38)                                                                      | 31          | 25          |
  | [array_u8_32_neq_deref](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L44)                                                                | 35          | 29          |
  | [array_u16_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L49)                                                                     | 33          | 27          |
  | [array_u32_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L54)                                                                     | 34          | 28          |
  | [array_u64_32_neq](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_neq.rs#L59)                                                                     | 34          | 28          |

  ### 10.3 Partial Eq Primitives

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L4)                                                                             | 7           | 1           |
  | [u16](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L9)                                                                            | 7           | 1           |
  | [u32](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L14)                                                                           | 7           | 1           |
  | [u64](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L19)                                                                           | 7           | 1           |
  | [u128](https://github.com/Lightprotocol/cu-library/blob/master/src/partial_eq/partial_eq_primitives.rs#L24)                                                                          | 7           | 1           |

## 11. Pinocchio Ops

  ### 11.1 Msg

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [msg10_chars](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/msg.rs#L5)                                                                                   | 110         | 104         |

  ### 11.2 Sysvar Clock

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [clock_get_slot](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/sysvar_clock.rs#L5)                                                                       | 171         | 165         |

  ### 11.3 Sysvar Rent

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sysvar_rent_exemption_165](https://github.com/Lightprotocol/cu-library/blob/master/src/pinocchio_ops/sysvar_rent.rs#L5)                                                             | 151         | 145         |

## 12. Saturating Math

  ### 12.1 Saturating Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L4)                                                                           | 7           | 1           |
  | [add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L11)                                                                         | 7           | 1           |
  | [add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L18)                                                                         | 7           | 1           |
  | [add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L25)                                                                         | 7           | 1           |
  | [add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_add.rs#L32)                                                                        | 8           | 2           |

  ### 12.2 Saturating Mul

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L4)                                                                           | 7           | 1           |
  | [mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L11)                                                                         | 7           | 1           |
  | [mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L18)                                                                         | 7           | 1           |
  | [mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L25)                                                                         | 7           | 1           |
  | [mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_mul.rs#L32)                                                                        | 8           | 2           |

  ### 12.3 Saturating Sub

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L4)                                                                           | 7           | 1           |
  | [sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L11)                                                                         | 7           | 1           |
  | [sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L18)                                                                         | 7           | 1           |
  | [sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L25)                                                                         | 7           | 1           |
  | [sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/saturating_math/saturating_sub.rs#L32)                                                                        | 8           | 2           |

## 13. Serialization

  ### 13.1 Compressed Account Info

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [borsh_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L188)                                                       | 427         | 421         |
  | [zero_copy_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L196)                                                   | 130         | 124         |
  | [wincode_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L205)                                                     | 1408        | 1402        |
  | [bincode_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L212)                                                     | 3230        | 3224        |
  | [borsh1_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L219)                                                      | 427         | 421         |
  | [rkyv_zero_copy_deserialize](https://github.com/Lightprotocol/cu-library/blob/master/src/serialization/compressed_account_info.rs#L226)                                              | 178         | 172         |

## 14. Solana Ops

  ### 14.1 Msg

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [msg10_chars](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/msg.rs#L5)                                                                                      | 110         | 104         |

  ### 14.2 Msg Program Id

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [msg_program_id](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/msg_program_id.rs#L5)                                                                        | 6954        | 6948        |

  ### 14.3 Pubkey New From Array

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [pubkey_new_from_array](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/pubkey_new_from_array.rs#L5)                                                          | 15          | 9           |

  ### 14.4 Pubkey To Bytes

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [pubkey_to_bytes](https://github.com/Lightprotocol/cu-library/blob/master/src/solana_ops/pubkey_to_bytes.rs#L6)                                                                      | 15          | 9           |

## 15. Std Math

  ### 15.1 Add Assign

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L4)                                                                               | 7           | 1           |
  | [add_assign_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L12)                                                                             | 7           | 1           |
  | [add_assign_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L20)                                                                             | 7           | 1           |
  | [add_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L28)                                                                             | 7           | 1           |
  | [add_assign_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/add_assign.rs#L36)                                                                            | 9           | 3           |

  ### 15.2 Std Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L4)                                                                                         | 6           | 0           |
  | [add_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L11)                                                                                       | 6           | 0           |
  | [add_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L18)                                                                                       | 6           | 0           |
  | [add_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L25)                                                                                       | 6           | 0           |
  | [add_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_add.rs#L32)                                                                                      | 6           | 0           |

  ### 15.3 Std Div

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [div_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L4)                                                                                         | 6           | 0           |
  | [div_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L11)                                                                                       | 6           | 0           |
  | [div_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L18)                                                                                       | 6           | 0           |
  | [div_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L25)                                                                                       | 6           | 0           |
  | [div_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_div.rs#L32)                                                                                      | 6           | 0           |

  ### 15.4 Std Mul

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [mul_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L4)                                                                                         | 6           | 0           |
  | [mul_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L11)                                                                                       | 6           | 0           |
  | [mul_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L18)                                                                                       | 6           | 0           |
  | [mul_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L25)                                                                                       | 6           | 0           |
  | [mul_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_mul.rs#L32)                                                                                      | 6           | 0           |

  ### 15.5 Std Sub

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L4)                                                                                         | 6           | 0           |
  | [sub_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L11)                                                                                       | 6           | 0           |
  | [sub_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L18)                                                                                       | 6           | 0           |
  | [sub_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L25)                                                                                       | 6           | 0           |
  | [sub_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/std_sub.rs#L32)                                                                                      | 6           | 0           |

  ### 15.6 Sub Assign

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [sub_assign_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L4)                                                                               | 7           | 1           |
  | [sub_assign_u16](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L12)                                                                             | 7           | 1           |
  | [sub_assign_u32](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L20)                                                                             | 7           | 1           |
  | [sub_assign_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L28)                                                                             | 7           | 1           |
  | [sub_assign_u128](https://github.com/Lightprotocol/cu-library/blob/master/src/std_math/sub_assign.rs#L36)                                                                            | 9           | 3           |

## 16. Vec

  ### 16.1 Vec New

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_new](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_new.rs#L4)                                                                                              | 9           | 3           |

  ### 16.2 Vec Push

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [push_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L5)                                                                                            | 73          | 67          |
  | [push_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L12)                                                                                          | 78          | 72          |
  | [push_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L19)                                                                                       | 85          | 79          |
  | [push_10_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L26)                                                                                        | 202         | 196         |
  | [push_10_u64](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L35)                                                                                       | 313         | 307         |
  | [push_10_pubkey](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L44)                                                                                    | 380         | 374         |
  | [push_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L53)                                                                             | 120         | 114         |
  | [push_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L60)                                                                            | 120         | 114         |
  | [push_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L67)                                                                         | 128         | 122         |
  | [push_10_u8_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L74)                                                                          | 153         | 147         |
  | [push_10_u64_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L83)                                                                         | 149         | 143         |
  | [push_10_pubkey_with_capacity](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_push.rs#L92)                                                                      | 218         | 212         |

  ### 16.3 Vec With Capacity

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [u8_with_capacity_10](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_with_capacity.rs#L4)                                                                       | 113         | 107         |
  | [u8_with_capacity_100](https://github.com/Lightprotocol/cu-library/blob/master/src/vec/vec_with_capacity.rs#L9)                                                                      | 113         | 107         |

