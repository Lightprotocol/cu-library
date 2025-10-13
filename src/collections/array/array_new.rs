use light_program_profiler::profile;

#[profile]
pub fn new() -> [u8; 32] {
    [0u8; 32]
}
