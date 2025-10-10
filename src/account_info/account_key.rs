use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[profile]
pub fn key(account: &AccountInfo) -> &Pubkey {
    account.key()
}