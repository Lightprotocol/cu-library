use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

#[profile]
pub fn account_info_realloc(account: &AccountInfo, new_len: usize) -> Result<(), ProgramError> {
    account.realloc(new_len, false)
}

#[profile]
pub fn account_info_close(account: &AccountInfo) -> Result<(), ProgramError> {
    account.close()
}

#[profile]
pub fn account_info_close_unchecked(account: &AccountInfo) {
    unsafe { account.close_unchecked() }
}
