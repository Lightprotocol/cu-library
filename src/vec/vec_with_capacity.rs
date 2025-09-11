use light_program_profiler::profile;

#[profile]
pub fn vec_u8_with_capacity_10() -> Vec<u8> {
    Vec::with_capacity(10)
}

#[profile]
pub fn vec_u8_with_capacity_100() -> Vec<u8> {
    Vec::with_capacity(100)
}
