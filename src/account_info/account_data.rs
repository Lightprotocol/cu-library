use light_program_profiler::profile;
use pinocchio::account_info::AccountInfo;

#[profile]
pub fn account_info_data_len(account: &AccountInfo) -> usize {
    account.data_len()
}

#[profile]
pub fn account_info_lamports(account: &AccountInfo) -> u64 {
    account.lamports()
}