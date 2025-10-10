use light_program_profiler::profile;

// u16 cast conversions
#[profile]
pub fn u16_as_u8(val: u16) -> u8 {
    val as u8
}

#[profile]
pub fn u16_as_u32(val: u16) -> u32 {
    val as u32
}

#[profile]
pub fn u16_as_u64(val: u16) -> u64 {
    val as u64
}

#[profile]
pub fn u16_as_usize(val: u16) -> usize {
    val as usize
}
