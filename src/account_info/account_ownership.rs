use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[profile]
pub fn account_info_is_owned_by(account: &AccountInfo, program: &Pubkey) -> bool {
    account.is_owned_by(program)
}

#[profile]
pub fn account_info_assign(account: &AccountInfo, new_owner: &Pubkey) {
    unsafe { account.assign(new_owner) }
}