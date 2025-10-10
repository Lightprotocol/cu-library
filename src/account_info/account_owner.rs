use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[profile]
pub fn owner(account: &AccountInfo) -> &Pubkey {
    account.owner()
}
