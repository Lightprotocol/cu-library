use light_program_profiler::profile;
use smallvec::SmallVec;

#[profile]
pub fn u8_with_capacity_10() -> SmallVec<[u8; 10]> {
    SmallVec::new()
}

#[profile]
pub fn u8_with_capacity_128() -> SmallVec<[u8; 128]> {
    SmallVec::new()
}
