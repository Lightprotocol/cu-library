use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[profile]
pub fn is_owned_by(account: &AccountInfo, program: &Pubkey) -> bool {
    account.is_owned_by(program)
}

#[profile]
pub fn assign(account: &AccountInfo, new_owner: &Pubkey) {
    unsafe { account.assign(new_owner) }
}