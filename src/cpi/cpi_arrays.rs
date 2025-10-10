use light_program_profiler::profile;
use pinocchio::{account_info::AccountInfo, instruction::AccountMeta};

#[profile]
pub fn account_meta_array_10(accounts: &[AccountInfo]) -> [AccountMeta<'_>; 10] {
    [
        AccountMeta {
            pubkey: accounts[0].key(),
            is_signer: accounts[0].is_signer(),
            is_writable: accounts[0].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[1].key(),
            is_signer: accounts[1].is_signer(),
            is_writable: accounts[1].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[2].key(),
            is_signer: accounts[2].is_signer(),
            is_writable: accounts[2].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[3].key(),
            is_signer: accounts[3].is_signer(),
            is_writable: accounts[3].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[4].key(),
            is_signer: accounts[4].is_signer(),
            is_writable: accounts[4].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[5].key(),
            is_signer: accounts[5].is_signer(),
            is_writable: accounts[5].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[6].key(),
            is_signer: accounts[6].is_signer(),
            is_writable: accounts[6].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[7].key(),
            is_signer: accounts[7].is_signer(),
            is_writable: accounts[7].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[8].key(),
            is_signer: accounts[8].is_signer(),
            is_writable: accounts[8].is_writable(),
        },
        AccountMeta {
            pubkey: accounts[9].key(),
            is_signer: accounts[9].is_signer(),
            is_writable: accounts[9].is_writable(),
        },
    ]
}

#[profile]
pub fn account_info_array_10_ref(accounts: &[AccountInfo]) -> [&AccountInfo; 10] {
    [
        &accounts[0],
        &accounts[1],
        &accounts[2],
        &accounts[3],
        &accounts[4],
        &accounts[5],
        &accounts[6],
        &accounts[7],
        &accounts[8],
        &accounts[9],
    ]
}

#[profile]
pub fn account_info_array_10_clone(accounts: &[AccountInfo]) -> [AccountInfo; 10] {
    [
        accounts[0].clone(),
        accounts[1].clone(),
        accounts[2].clone(),
        accounts[3].clone(),
        accounts[4].clone(),
        accounts[5].clone(),
        accounts[6].clone(),
        accounts[7].clone(),
        accounts[8].clone(),
        accounts[9].clone(),
    ]
}

#[profile]
pub fn account_info_array_10_move(accounts: &[AccountInfo]) -> [AccountInfo; 10] {
    [
        accounts[0],
        accounts[1],
        accounts[2],
        accounts[3],
        accounts[4],
        accounts[5],
        accounts[6],
        accounts[7],
        accounts[8],
        accounts[9],
    ]
}
