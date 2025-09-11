use light_program_profiler::profile;

#[profile]
pub fn partial_eq_u8(a: u8, b: u8) -> bool {
    a == b
}

#[profile]
pub fn partial_eq_u16(a: u16, b: u16) -> bool {
    a == b
}

#[profile]
pub fn partial_eq_u32(a: u32, b: u32) -> bool {
    a == b
}

#[profile]
pub fn partial_eq_u64(a: u64, b: u64) -> bool {
    a == b
}

#[profile]
pub fn partial_eq_u128(a: u128, b: u128) -> bool {
    a == b
}