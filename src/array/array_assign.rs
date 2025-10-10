use light_program_profiler::profile;
use pinocchio::pubkey::Pubkey;

#[profile]
pub fn assign_u8() -> [u8; 10] {
    let mut arr = [0u8; 10];
    arr[0] = 42u8;
    arr
}

#[profile]
pub fn assign_u64() -> [u64; 10] {
    let mut arr = [0u64; 10];
    arr[0] = 12345678u64;
    arr
}

#[profile]
pub fn assign_pubkey(program_id: &Pubkey) -> [Pubkey; 10] {
    let mut arr = [*program_id; 10];
    arr[0] = *program_id;
    arr
}

#[profile]
pub fn assign_10_u8() -> [u8; 10] {
    let mut arr = [0u8; 10];
    for i in 0..10 {
        arr[i] = i as u8;
    }
    arr
}

#[profile]
pub fn assign_10_u64() -> [u64; 10] {
    let mut arr = [0u64; 10];
    for i in 0..10 {
        arr[i] = i as u64;
    }
    arr
}

#[profile]
pub fn assign_10_pubkey(program_id: &Pubkey) -> [Pubkey; 10] {
    let mut arr = [*program_id; 10];
    for i in 0..10 {
        arr[i] = *program_id;
    }
    arr
}
