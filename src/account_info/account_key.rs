use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[profile]
pub fn account_info_key(account: &AccountInfo) -> &Pubkey {
    account.key()
}