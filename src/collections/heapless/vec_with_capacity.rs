use heapless::Vec;
use light_program_profiler::profile;

#[profile]
pub fn u8_with_capacity_10() -> Vec<u8, 10> {
    Vec::new()
}

#[profile]
pub fn u8_with_capacity_100() -> Vec<u8, 100> {
    Vec::new()
}
