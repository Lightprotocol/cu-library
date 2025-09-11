use light_program_profiler::profile;

// Reference version for u8
#[profile]
pub fn partial_eq_array_u8_32_ref(a: &[u8; 32], b: &[u8; 32]) -> bool {
    a == b
}

// Value version for u8
#[profile]
pub fn partial_eq_array_u8_32(a: [u8; 32], b: [u8; 32]) -> bool {
    a == b
}

#[profile]
pub fn partial_eq_array_u16_32(a: &[u16; 32], b: &[u16; 32]) -> bool {
    a == b
}

#[profile]
pub fn partial_eq_array_u32_32(a: &[u32; 32], b: &[u32; 32]) -> bool {
    a == b
}

#[profile]
pub fn partial_eq_array_u64_32(a: &[u64; 32], b: &[u64; 32]) -> bool {
    a == b
}
