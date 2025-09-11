use light_program_profiler::profile;
use arrayvec::ArrayVec;

#[profile]
pub fn array_vec_new() -> ArrayVec<u8, 32> {
    ArrayVec::new()
}