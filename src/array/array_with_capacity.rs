use light_program_profiler::profile;

#[profile]
pub fn with_capacity_10() -> [u8; 10] {
    [0u8; 10]
}

#[profile]
pub fn with_capacity_100() -> [u8; 100] {
    [0u8; 100]
}
