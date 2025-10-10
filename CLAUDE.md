# CU Library

## Purpose

A benchmarking repository for measuring Compute Units (CU) and heap memory consumption of common Solana runtime operations. This library uses pinocchio with u16 instruction discriminators and light-program-profiler to capture precise performance metrics for on-chain operations.

## Benchmark Output

Running `cargo test-sbf` generates `README.md` with:
- **Table of Contents** with links to each category
- **Definitions** section explaining CU metrics
- **Results organized by category** based on directory structure and file names
- Each category contains subsections by file, with tables showing function benchmarks

Example structure:
```markdown
## Table of Contents

**[1. Baseline](#1-baseline)**

**[2. Checked Math](#2-checked-math)**

## Definitions

- **CU Consumed**: Total compute units consumed by the profiled function
- **CU Adjusted**: Actual operation cost with baseline profiling overhead subtracted (CU Consumed - Baseline CU)
- **Baseline CU**: CU consumed by an empty profiled function (`#[profile]` macro overhead)

## 1. Baseline

  ### 1.1 Lib

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [baseline_empty_function](https://github.com/Lightprotocol/cu-library/blob/master/src/lib.rs#L102)                                                                                   | 6           | N/A         |

## 2. Checked Math

  ### 2.1 Checked Add

  | Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |
  |-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|
  | [add_u8](https://github.com/Lightprotocol/cu-library/blob/master/src/checked_math/checked_add.rs#L4)                                                                                 | 7           | 1           |
```

## Adding New Profiling Cases

### Directory Structure

Organize profiling functions by operation category:

- `src/pinocchio_ops/` - Pinocchio-specific operations (e.g., msg!, pubkey operations)
- `src/solana_ops/` - Solana SDK operations (e.g., solana_msg::msg!)
- `src/checked_math/` - Checked arithmetic operations
- `src/unchecked_math/` - Unchecked arithmetic operations

### Steps to Add a New Profiling Case

1. **Create the operation file** in the appropriate directory:

```rust
// src/checked_math/add.rs
use light_program_profiler::profile;
use pinocchio::ProgramResult;

#[profile]
pub fn checked_add_u64() -> ProgramResult {
    let a: u64 = 1000;
    let b: u64 = 2000;
    let _result = a.checked_add(b);
    Ok(())
}
```

**IMPORTANT: Preventing Compiler Optimization**

To ensure accurate CU measurements, functions should return actual values (not just `ProgramResult`) and these values must be printed in `lib.rs`:

```rust
// Return the actual computed value
#[profile]
pub fn add_u64() -> Option<u64> {
    let a: u64 = 1000;
    let b: u64 = 2000;
    a.checked_add(b)
}
```

Then in `lib.rs`, print the full value:

```rust
CuLibraryInstruction::CheckedAddU64 => {
    let res = checked_math::checked_add::add_u64();  // Use module-qualified path
    solana_msg::msg!("result: {:?}", res);  // Print the actual value (Option type)
}
```

**Note:** If a benchmark shows ≤8 CU, it's likely being optimized away. Ensure you:
- Return the actual computed value from the function
- Print the full value (not just `.len()` or similar)
- Use the value in a way that prevents optimization

**Using Pubkeys in Benchmarks:**

When your benchmark function needs a `Pubkey` parameter, always pass the `program_id` from the `process_instruction` function:

```rust
// In your benchmark function
#[profile]
pub fn push_pubkey(program_id: &Pubkey) -> ArrayVec<Pubkey, 10> {
    let mut vec = ArrayVec::new();
    vec.push(*program_id);
    vec
}

// In lib.rs process_instruction
CuLibraryInstruction::ArrayvecPushPubkey => {
    let res = arrayvec::vec_push::push_pubkey(program_id);  // Pass program_id with module path
    solana_msg::msg!("vec: {:?}", res.as_slice());
}
```

**Benchmarking Serialization/Deserialization:**

For benchmarks that need test data (like serialization), create helper functions that are NOT profiled to prepare the data.

**Best Practice**: Profile multiple serialization methods (Borsh, Zero-Copy, etc.) for the same data structure to compare performance:

```rust
// In your benchmark file (e.g., src/serialization/my_struct.rs)
use borsh::{BorshDeserialize, BorshSerialize};
use light_program_profiler::profile;
use light_zero_copy::{traits::ZeroCopyAt, ZeroCopy};
use pinocchio::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, ZeroCopy, Debug)]
#[repr(C)]
pub struct MyStruct {
    pub field1: u64,
    pub field2: [u8; 32],
}

// Helper function to create test data - NOT profiled
pub fn serialize_my_struct() -> Vec<u8> {
    let test_data = MyStruct {
        field1: 1000,
        field2: [1u8; 32],
    };
    borsh::to_vec(&test_data).unwrap()
}

// Profile Borsh deserialization
#[profile]
pub fn borsh_deserialize(serialized_data: &[u8]) -> Result<MyStruct, ProgramError> {
    MyStruct::try_from_slice(serialized_data)
        .map_err(|_| ProgramError::InvalidAccountData)
}

// Profile Zero-Copy deserialization for comparison
#[profile]
pub fn zero_copy_deserialize<'a>(
    serialized_data: &'a [u8],
) -> Result<<MyStruct as ZeroCopyAt<'a>>::ZeroCopyAt, ProgramError> {
    Ok(MyStruct::zero_copy_at(serialized_data)
        .map_err(|e| ProgramError::from((u32::from(e)) as u64))?
        .0)
}

// In lib.rs - Add both enum variants
#[repr(u16)]
pub enum CuLibraryInstruction {
    // ...
    MyStructBorshDeserialize = 230,
    MyStructZeroCopyDeserialize = 231,
    // ...
}

// In lib.rs process_instruction - Profile both methods
CuLibraryInstruction::MyStructBorshDeserialize => {
    let data = serialization::my_struct::serialize_my_struct();
    let res = serialization::my_struct::borsh_deserialize(data.as_slice())?;
    solana_msg::msg!("Borsh deserialized: {:?}", res.field1);
}
CuLibraryInstruction::MyStructZeroCopyDeserialize => {
    let data = serialization::my_struct::serialize_my_struct();
    let res = serialization::my_struct::zero_copy_deserialize(data.as_slice())?;
    solana_msg::msg!("Zero-copy deserialized: {:?}", res.field1);
}
```

**Key Patterns**:
- Only the operation you want to measure should be `#[profile]` annotated. Data preparation helpers should not be profiled.
- Always benchmark multiple serialization methods for comparison (Borsh vs Zero-Copy) to identify the most efficient approach.
- Zero-Copy deserialization is typically much faster than Borsh (see README results for concrete CU comparisons).

2. **Add module declaration** to the directory's `mod.rs`:

```rust
// src/checked_math/mod.rs
pub mod checked_add;
```

3. **Register in main lib.rs**:

```rust
// src/lib.rs - Add to module declarations at top of file
pub mod checked_math;

// Add new variant to the enum
#[repr(u16)]
pub enum CuLibraryInstruction {
    Baseline = 0,
    Msg10 = 1,
    // ... existing variants ...
    CheckedAddU64 = 37,  // Find next available number
    // ...
}

// Update TryFrom implementation
impl TryFrom<&[u8]> for CuLibraryInstruction {
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let discriminator = u16::from_le_bytes([value[0], value[1]]);
        match discriminator {
            0 => Ok(CuLibraryInstruction::Baseline),
            1 => Ok(CuLibraryInstruction::Msg10),
            // ... existing cases ...
            37 => Ok(CuLibraryInstruction::CheckedAddU64),  // New case
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

// Add to process_instruction match
pub fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let instruction = CuLibraryInstruction::try_from(instruction_data)?;
    match instruction {
        CuLibraryInstruction::Baseline => {
            baseline_empty_function();
            solana_msg::msg!("baseline complete");
        }
        CuLibraryInstruction::Msg10 => pinocchio_ops::msg::msg10_chars()?,
        // ... existing cases ...
        CuLibraryInstruction::CheckedAddU64 => {
            let res = checked_math::checked_add::add_u64();
            solana_msg::msg!("result: {}", res);
        }
        // ...
    }
    Ok(())
}
```

4. **Add test case** in `tests/bench.rs`:

```rust
let instructions = vec![
    CuLibraryInstruction::Msg10,
    CuLibraryInstruction::CheckedAddU64,  // New test
];
```

### Naming Conventions

- **Function names**: Use descriptive snake_case (e.g., `msg10_chars`, `add_u64`, `push_pubkey`)
  - Function names should describe the operation being tested
  - No need for category prefixes - the directory determines the category
- **File names**: Match the operation being tested (e.g., `checked_add.rs`, `msg.rs`, `vec_push.rs`)
- **Directory names**: Use descriptive names for operation categories (e.g., `pinocchio_ops`, `solana_ops`, `checked_math`)

### Category Organization

The benchmark results are automatically organized in README.md by **directory structure**:

1. **Directory** determines the main category section (e.g., `src/checked_math/` → "## Checked Math")
2. **File** determines the subsection (e.g., `checked_add.rs` → "### Checked Add")
3. **Function** appears as a row in the table with a GitHub link to the source

Example directory structure:
```
src/
├── baseline/           → "## Baseline"
├── checked_math/       → "## Checked Math"
│   ├── checked_add.rs  →   "### Checked Add"
│   ├── checked_sub.rs  →   "### Checked Sub"
│   └── checked_mul.rs  →   "### Checked Mul"
├── pinocchio_ops/      → "## Pinocchio Ops"
│   ├── msg.rs          →   "### Msg"
│   └── sysvar_clock.rs →   "### Sysvar Clock"
└── solana_ops/         → "## Solana Ops"
    └── msg.rs          →   "### Msg"
```

This organization makes it easy to compare similar operations across different implementations and keeps related benchmarks grouped together.
