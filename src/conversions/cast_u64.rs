use light_program_profiler::profile;

// u64 cast conversions
#[profile]
pub fn conversions_u64_as_u8(val: u64) -> u8 {
    val as u8
}

#[profile]
pub fn conversions_u64_as_u16(val: u64) -> u16 {
    val as u16
}

#[profile]
pub fn conversions_u64_as_u32(val: u64) -> u32 {
    val as u32
}

#[profile]
pub fn conversions_u64_as_usize(val: u64) -> usize {
    val as usize
}