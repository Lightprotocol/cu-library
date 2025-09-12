use light_program_profiler::profile;
use pinocchio::program_error::ProgramError;

// Slice to [u8; 32] conversions with different error handling

#[profile]
pub fn conversions_try_into_slice_to_array_32_unwrap(slice: &[u8]) -> [u8; 32] {
    slice.try_into().unwrap()
}

#[profile]
pub fn conversions_try_into_slice_to_array_32_map_err(
    slice: &[u8],
) -> Result<[u8; 32], ProgramError> {
    slice.try_into().map_err(|_| ProgramError::InvalidArgument)
}
