use light_program_profiler::profile;
use pinocchio::account_info::AccountInfo;

#[profile]
pub fn data_len(account: &AccountInfo) -> usize {
    account.data_len()
}

#[profile]
pub fn lamports(account: &AccountInfo) -> u64 {
    account.lamports()
}