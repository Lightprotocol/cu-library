use light_program_profiler::profile;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};
use smallvec::SmallVec;

// Helpers to create vecs - NOT profiled
pub fn create_empty_u8_vec() -> SmallVec<[u8; 10]> {
    SmallVec::new()
}

pub fn create_empty_u64_vec() -> SmallVec<[u64; 10]> {
    SmallVec::new()
}

pub fn create_empty_pubkey_vec() -> SmallVec<[Pubkey; 10]> {
    SmallVec::new()
}

#[profile]
pub fn push_u8(vec: &mut SmallVec<[u8; 10]>) {
    vec.push(42u8);
}

#[profile]
pub fn push_u64(vec: &mut SmallVec<[u64; 10]>) {
    vec.push(12345678u64);
}

#[profile]
pub fn push_pubkey(vec: &mut SmallVec<[Pubkey; 10]>, program_id: &Pubkey) {
    vec.push(*program_id);
}

#[profile]
pub fn push_10_u8(vec: &mut SmallVec<[u8; 10]>) {
    for i in 0..10 {
        vec.push(i as u8);
    }
}

#[profile]
pub fn push_10_u64(vec: &mut SmallVec<[u64; 10]>) {
    for i in 0..10 {
        vec.push(i as u64);
    }
}

#[profile]
pub fn push_10_pubkey(vec: &mut SmallVec<[Pubkey; 10]>, program_id: &Pubkey) -> Result<(), ProgramError> {
    for _ in 0..10 {
        vec.push(*program_id);
    }
    Ok(())
}
