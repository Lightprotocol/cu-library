use light_program_profiler::profile;
use pinocchio::program_error::ProgramError;

// usize to u64 conversions
#[profile]
pub fn conversions_try_into_usize_to_u64_unwrap(val: usize) -> u64 {
    val.try_into().unwrap()
}

#[profile]
pub fn conversions_try_into_usize_to_u64_map_err(val: usize) -> Result<u64, ProgramError> {
    val.try_into().map_err(|_| ProgramError::InvalidArgument)
}
