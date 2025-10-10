use light_program_profiler::profile;

// Non-equal comparisons for primitives
#[profile]
pub fn u8_neq(a: u8, b: u8) -> bool {
    a == b
}

#[profile]
pub fn u16_neq(a: u16, b: u16) -> bool {
    a == b
}

#[profile]
pub fn u32_neq(a: u32, b: u32) -> bool {
    a == b
}

#[profile]
pub fn u64_neq(a: u64, b: u64) -> bool {
    a == b
}

#[profile]
pub fn u128_neq(a: u128, b: u128) -> bool {
    a == b
}

// Non-equal comparisons for arrays (last element different)
// Reference version
#[profile]
pub fn array_u8_32_neq_ref(a: &[u8; 32], b: &[u8; 32]) -> bool {
    a == b
}

// Value version
#[profile]
pub fn array_u8_32_neq(a: [u8; 32], b: [u8; 32]) -> bool {
    a == b
}

// Dereference version - dereferences inside the profiled function
#[profile]
pub fn array_u8_32_neq_deref(a: &[u8; 32], b: &[u8; 32]) -> bool {
    *a == *b
}

#[profile]
pub fn array_u16_32_neq(a: &[u16; 32], b: &[u16; 32]) -> bool {
    a == b
}

#[profile]
pub fn array_u32_32_neq(a: &[u32; 32], b: &[u32; 32]) -> bool {
    a == b
}

#[profile]
pub fn array_u64_32_neq(a: &[u64; 32], b: &[u64; 32]) -> bool {
    a == b
}