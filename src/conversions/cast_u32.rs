use light_program_profiler::profile;

// u32 cast conversions
#[profile]
pub fn conversions_u32_as_u8(val: u32) -> u8 {
    val as u8
}

#[profile]
pub fn conversions_u32_as_u16(val: u32) -> u16 {
    val as u16
}

#[profile]
pub fn conversions_u32_as_u64(val: u32) -> u64 {
    val as u64
}

#[profile]
pub fn conversions_u32_as_usize(val: u32) -> usize {
    val as usize
}