use light_program_profiler::profile;
use pinocchio::account_info::AccountInfo;

#[profile]
pub fn is_signer(account: &AccountInfo) -> bool {
    account.is_signer()
}

#[profile]
pub fn is_writable(account: &AccountInfo) -> bool {
    account.is_writable()
}

#[profile]
pub fn executable(account: &AccountInfo) -> bool {
    account.executable()
}

#[profile]
pub fn data_is_empty(account: &AccountInfo) -> bool {
    account.data_is_empty()
}