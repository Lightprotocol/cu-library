use light_program_profiler::profile;
use arrayvec::ArrayVec;

#[profile]
pub fn arrayvec_with_capacity_10() -> ArrayVec<u8, 10> {
    ArrayVec::new()
}

#[profile]
pub fn arrayvec_with_capacity_100() -> ArrayVec<u8, 100> {
    ArrayVec::new()
}