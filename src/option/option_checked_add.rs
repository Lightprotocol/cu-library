use light_program_profiler::profile;
use pinocchio::program_error::ProgramError;

// Option handling with checked_add_u8

#[profile]
pub fn option_checked_add_u8_unwrap(val: u8) -> u8 {
    let option = val.checked_add(1);
    option.unwrap()
}

#[profile]
pub fn option_checked_add_u8_ok_or(val: u8) -> Result<u8, ProgramError> {
    let option = val.checked_add(1);
    option.ok_or(ProgramError::InvalidArgument)
}

#[profile]
pub fn option_checked_add_u8_ok_or_else(val: u8) -> Result<u8, ProgramError> {
    let option = val.checked_add(1);
    option.ok_or_else(|| ProgramError::InvalidArgument)
}

#[profile]
pub fn option_checked_add_u8_unwrap_or_default(val: u8) -> u8 {
    let option = val.checked_add(1);
    option.unwrap_or_default()
}

#[profile]
pub fn option_checked_add_u8_unwrap_or(val: u8) -> u8 {
    let option = val.checked_add(1);
    option.unwrap_or(1)
}