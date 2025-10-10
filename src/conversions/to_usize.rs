use light_program_profiler::profile;
use pinocchio::program_error::ProgramError;

// u64 to usize conversions
#[profile]
pub fn try_into_u64_to_usize_unwrap(val: u64) -> usize {
    val.try_into().unwrap()
}

#[profile]
pub fn try_into_u64_to_usize_map_err(val: u64) -> Result<usize, ProgramError> {
    val.try_into().map_err(|_| ProgramError::InvalidArgument)
}

// u32 to usize conversions
#[profile]
pub fn try_into_u32_to_usize_unwrap(val: u32) -> usize {
    val.try_into().unwrap()
}

#[profile]
pub fn try_into_u32_to_usize_map_err(val: u32) -> Result<usize, ProgramError> {
    val.try_into().map_err(|_| ProgramError::InvalidArgument)
}

// u16 to usize conversions
#[profile]
pub fn try_into_u16_to_usize_unwrap(val: u16) -> usize {
    val.try_into().unwrap()
}

#[profile]
pub fn try_into_u16_to_usize_map_err(val: u16) -> Result<usize, ProgramError> {
    val.try_into().map_err(|_| ProgramError::InvalidArgument)
}

// u8 to usize conversions
#[profile]
pub fn try_into_u8_to_usize_unwrap(val: u8) -> usize {
    val.try_into().unwrap()
}

#[profile]
pub fn try_into_u8_to_usize_map_err(val: u8) -> Result<usize, ProgramError> {
    val.try_into().map_err(|_| ProgramError::InvalidArgument)
}
