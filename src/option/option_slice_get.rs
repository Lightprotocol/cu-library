use light_program_profiler::profile;
use pinocchio::program_error::ProgramError;

// Option handling with slice.get(0) for [u8; 32] arrays

#[profile]
pub fn slice_get_array_unwrap(arrays: &[[u8; 32]]) -> [u8; 32] {
    let option = arrays.get(0);
    *option.unwrap()
}

#[profile]
pub fn slice_get_array_ok_or(arrays: &[[u8; 32]]) -> Result<[u8; 32], ProgramError> {
    let option = arrays.get(0);
    option.copied().ok_or(ProgramError::InvalidArgument)
}

#[profile]
pub fn slice_get_array_ok_or_else(arrays: &[[u8; 32]]) -> Result<[u8; 32], ProgramError> {
    let option = arrays.get(0);
    option.copied().ok_or_else(|| ProgramError::InvalidArgument)
}

#[profile]
pub fn slice_get_array_unwrap_or_default(arrays: &[[u8; 32]]) -> [u8; 32] {
    let option = arrays.get(0);
    option.copied().unwrap_or_default()
}

#[profile]
pub fn slice_get_array_unwrap_or(arrays: &[[u8; 32]]) -> [u8; 32] {
    let option = arrays.get(0);
    option.copied().unwrap_or([1u8; 32])
}