use light_program_profiler::profile;
use pinocchio::pubkey::Pubkey;

// Option handling with if let Some pattern

#[profile]
pub fn if_let_some_u8(option: Option<u8>) -> u8 {
    if let Some(value) = option {
        value
    } else {
        panic!("Expected Some value")
    }
}

#[profile]
pub fn if_let_some_array(option: Option<[u8; 32]>) -> [u8; 32] {
    if let Some(value) = option {
        value
    } else {
        panic!("Expected Some array")
    }
}

#[profile]
pub fn if_let_some_pubkey(option: Option<Pubkey>) -> Pubkey {
    if let Some(value) = option {
        value
    } else {
        panic!("Expected Some pubkey")
    }
}

#[profile]
pub fn if_let_some_array_ref(option: Option<&[u8; 32]>) -> &[u8; 32] {
    if let Some(value) = option {
        value
    } else {
        panic!("Expected Some array ref")
    }
}
