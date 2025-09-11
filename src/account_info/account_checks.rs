use light_program_profiler::profile;
use pinocchio::account_info::AccountInfo;

#[profile]
pub fn account_info_is_signer(account: &AccountInfo) -> bool {
    account.is_signer()
}

#[profile]
pub fn account_info_is_writable(account: &AccountInfo) -> bool {
    account.is_writable()
}

#[profile]
pub fn account_info_executable(account: &AccountInfo) -> bool {
    account.executable()
}

#[profile]
pub fn account_info_data_is_empty(account: &AccountInfo) -> bool {
    account.data_is_empty()
}