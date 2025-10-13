use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

#[profile]
pub fn realloc(account: &AccountInfo, new_len: usize) -> Result<(), ProgramError> {
    account.resize(new_len)
}

#[profile]
pub fn close(account: &AccountInfo) -> Result<(), ProgramError> {
    account.close()
}

#[profile]
pub fn close_unchecked(account: &AccountInfo) {
    unsafe { account.close_unchecked() }
}
