use light_program_profiler::profile;

#[profile]
pub fn vec_new() -> Vec<u8> {
    Vec::new()
}