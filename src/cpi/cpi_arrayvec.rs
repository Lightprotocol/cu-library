use arrayvec::ArrayVec;
use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, instruction::AccountMeta};

#[profile]
pub fn cpi_arrayvec_push_account_meta_10(accounts: &[AccountInfo]) -> ArrayVec<AccountMeta<'_>, 10> {
    let mut vec = ArrayVec::new();
    for i in 0..10 {
        vec.push(AccountMeta {
            pubkey: accounts[i].key(),
            is_signer: accounts[i].is_signer(),
            is_writable: accounts[i].is_writable(),
        });
    }
    vec
}

#[profile]
pub fn cpi_arrayvec_push_account_info_10_ref(
    accounts: &[AccountInfo],
) -> ArrayVec<&AccountInfo, 10> {
    let mut vec = ArrayVec::new();
    for i in 0..10 {
        vec.push(&accounts[i]);
    }
    vec
}

#[profile]
pub fn cpi_arrayvec_push_account_info_10_clone(
    accounts: &[AccountInfo],
) -> ArrayVec<AccountInfo, 10> {
    let mut vec = ArrayVec::new();
    for i in 0..10 {
        vec.push(accounts[i].clone());
    }
    vec
}

#[profile]
pub fn cpi_arrayvec_push_account_info_10_move(
    accounts: &[AccountInfo],
) -> ArrayVec<AccountInfo, 10> {
    let mut vec = ArrayVec::new();
    for i in 0..10 {
        vec.push(accounts[i]);
    }
    vec
}
