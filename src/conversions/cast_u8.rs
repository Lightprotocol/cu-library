use light_program_profiler::profile;

// u8 cast conversions
#[profile]
pub fn u8_as_u16(val: u8) -> u16 {
    val as u16
}

#[profile]
pub fn u8_as_u32(val: u8) -> u32 {
    val as u32
}

#[profile]
pub fn u8_as_u64(val: u8) -> u64 {
    val as u64
}

#[profile]
pub fn u8_as_usize(val: u8) -> usize {
    val as usize
}
