use light_program_profiler::profile;
use smallvec::SmallVec;

#[profile]
pub fn u8_new() -> SmallVec<[u8; 32]> {
    SmallVec::new()
}
