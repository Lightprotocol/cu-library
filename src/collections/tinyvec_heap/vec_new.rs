use tinyvec::TinyVec;
use light_program_profiler::profile;

#[profile]
pub fn u8_new() -> TinyVec<[u8; 32]> {
    TinyVec::new()
}
