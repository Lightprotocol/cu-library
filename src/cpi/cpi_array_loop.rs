use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, instruction::AccountMeta};

#[profile]
pub fn cpi_account_meta_array_10_loop(accounts: &[AccountInfo]) -> [AccountMeta; 10] {
    // Create a static default pubkey to use as reference
    static DEFAULT_PUBKEY: [u8; 32] = [0; 32];

    // Use array::from_fn to initialize with default values
    let mut metas = core::array::from_fn(|_| AccountMeta {
        pubkey: &DEFAULT_PUBKEY,
        is_signer: false,
        is_writable: false,
    });

    for i in 0..10 {
        metas[i] = AccountMeta {
            pubkey: accounts[i].key(),
            is_signer: accounts[i].is_signer(),
            is_writable: accounts[i].is_writable(),
        };
    }

    metas
}

#[profile]
pub fn cpi_account_info_array_10_ref_loop(accounts: &[AccountInfo]) -> [&AccountInfo; 10] {
    // Create a reference to the first account as default
    let default_ref = &accounts[0];
    let mut refs = [default_ref; 10];

    for i in 0..10 {
        refs[i] = &accounts[i];
    }

    refs
}

#[profile]
pub fn cpi_account_info_array_10_clone_loop(accounts: &[AccountInfo]) -> [AccountInfo; 10] {
    // Use array::from_fn to initialize with clones of first account
    let mut clones = core::array::from_fn(|_| accounts[0].clone());

    // Reassign each element in the loop
    for i in 0..10 {
        clones[i] = accounts[i].clone();
    }

    clones
}

#[profile]
pub fn cpi_account_info_array_10_move_loop(accounts: &[AccountInfo]) -> [AccountInfo; 10] {
    // Use array::from_fn to initialize with the first account
    let mut moved = core::array::from_fn(|_| accounts[0]);

    // Reassign each element in the loop
    for i in 0..10 {
        moved[i] = accounts[i];
    }

    moved
}
