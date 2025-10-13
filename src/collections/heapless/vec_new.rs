use heapless::Vec;
use light_program_profiler::profile;

#[profile]
pub fn u8_new() -> Vec<u8, 32> {
    Vec::new()
}
