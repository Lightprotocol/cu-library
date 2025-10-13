use heapless::Vec;
use light_program_profiler::profile;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

// Helpers to create vecs - NOT profiled
pub fn create_empty_u8_vec() -> Vec<u8, 10> {
    Vec::new()
}

pub fn create_empty_u64_vec() -> Vec<u64, 10> {
    Vec::new()
}

pub fn create_empty_pubkey_vec() -> Vec<Pubkey, 10> {
    Vec::new()
}

#[profile]
pub fn push_u8(vec: &mut Vec<u8, 10>) {
    vec.push(42u8).unwrap();
}

#[profile]
pub fn push_u64(vec: &mut Vec<u64, 10>) {
    vec.push(12345678u64).unwrap();
}

#[profile]
pub fn push_pubkey(vec: &mut Vec<Pubkey, 10>, program_id: &Pubkey) {
    vec.push(*program_id).unwrap();
}

#[profile]
pub fn push_10_u8(vec: &mut Vec<u8, 10>) {
    for i in 0..10 {
        vec.push(i as u8).unwrap();
    }
}

#[profile]
pub fn push_10_u64(vec: &mut Vec<u64, 10>) {
    for i in 0..10 {
        vec.push(i as u64).unwrap();
    }
}

#[profile]
pub fn push_10_pubkey(vec: &mut Vec<Pubkey, 10>, program_id: &Pubkey) -> Result<(), ProgramError> {
    for _ in 0..10 {
        vec.push(*program_id).unwrap();
    }
    Ok(())
}
