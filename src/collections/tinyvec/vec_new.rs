use tinyvec::ArrayVec;
use light_program_profiler::profile;

#[profile]
pub fn u8_new() -> ArrayVec<[u8; 32]> {
    ArrayVec::new()
}
