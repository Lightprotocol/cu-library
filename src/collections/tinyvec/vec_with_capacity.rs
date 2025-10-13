use tinyvec::ArrayVec;
use light_program_profiler::profile;

#[profile]
pub fn u8_with_capacity_10() -> ArrayVec<[u8; 10]> {
    ArrayVec::new()
}

#[profile]
pub fn u8_with_capacity_100() -> ArrayVec<[u8; 100]> {
    ArrayVec::new()
}
