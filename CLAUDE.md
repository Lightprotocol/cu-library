# CU Library

## Purpose

A benchmarking repository for measuring Compute Units (CU) and heap memory consumption of common Solana runtime operations. This library uses pinocchio with u16 instruction discriminators and light-program-profiler to capture precise performance metrics for on-chain operations.

## Benchmark Output

Running `cargo test-sbf` generates `README.md` with results organized by category based on function name prefix:

```markdown
## Pinocchio

| Function                                    | CU Consumed | Heap Bytes Used |
|---------------------------------------------|-------------|-----------------|
| pinocchio_msg10_chars                       | 111         | 0               |

## Solana

| Function                                    | CU Consumed | Heap Bytes Used |
|---------------------------------------------|-------------|-----------------|
| solana_msg10_chars                          | 112         | 0               |
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

2. **Add module declaration** to the directory's `mod.rs`:

```rust
// src/checked_math/mod.rs
pub mod add;
```

3. **Register in main lib.rs**:

```rust
// src/lib.rs
pub mod checked_math;

#[repr(u16)]
pub enum CuLibraryInstruction {
    Msg10 = 1,
    CheckedAddU64 = 2,  // New instruction
}

// Update TryFrom implementation
impl TryFrom<&[u8]> for CuLibraryInstruction {
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let discriminator = u16::from_le_bytes([value[0], value[1]]);
        match discriminator {
            1 => Ok(CuLibraryInstruction::Msg10),
            2 => Ok(CuLibraryInstruction::CheckedAddU64),  // New case
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

// Add to process_instruction match
pub fn process_instruction(...) -> ProgramResult {
    let instruction = CuLibraryInstruction::try_from(instruction_data)?;
    match instruction {
        CuLibraryInstruction::Msg10 => pinocchio_msg10_chars(),
        CuLibraryInstruction::CheckedAddU64 => checked_add_u64(),  // New case
    }
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

- **Function names**: Use descriptive snake_case with category prefix (e.g., `pinocchio_msg10_chars`, `solana_msg10_chars`, `checked_add_u64`)
  - The prefix before the first underscore determines the category grouping in README.md
  - Functions starting with `pinocchio_` will be grouped under "## Pinocchio"
  - Functions starting with `solana_` will be grouped under "## Solana"
  - Functions starting with `checked_` will be grouped under "## Checked"
- **File names**: Match the operation being tested (e.g., `add.rs`, `mul.rs`, `hash.rs`)
- **Directory names**: Use category_ops pattern matching the function prefix (e.g., `pinocchio_ops`, `solana_ops`, `checked_math`)

### Category Organization

The benchmark results are automatically grouped in README.md by the function name prefix:

| Directory | Function Prefix | README Section |
|-----------|----------------|----------------|
| `src/pinocchio_ops/` | `pinocchio_*` | ## Pinocchio |
| `src/solana_ops/` | `solana_*` | ## Solana |
| `src/checked_math/` | `checked_*` | ## Checked |
| `src/unchecked_math/` | `unchecked_*` | ## Unchecked |
| `src/sysvar_ops/` | `sysvar_*` | ## Sysvar |

This organization makes it easy to compare similar operations across different implementations.
