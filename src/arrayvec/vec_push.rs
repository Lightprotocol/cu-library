use arrayvec::ArrayVec;
use light_program_profiler::profile;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

#[profile]
pub fn arrayvec_push_u8() -> ArrayVec<u8, 10> {
    let mut vec: ArrayVec<u8, 10> = ArrayVec::new();
    vec.push(42u8);
    vec
}

#[profile]
pub fn arrayvec_push_u64() -> ArrayVec<u64, 10> {
    let mut vec: ArrayVec<u64, 10> = ArrayVec::new();
    vec.push(12345678u64);
    vec
}

#[profile]
pub fn arrayvec_push_pubkey(program_id: &Pubkey) -> ArrayVec<Pubkey, 10> {
    let mut vec: ArrayVec<Pubkey, 10> = ArrayVec::new();
    vec.push(*program_id);
    vec
}

#[profile]
pub fn arrayvec_push_10_u8() -> ArrayVec<u8, 10> {
    let mut vec: ArrayVec<u8, 10> = ArrayVec::new();
    for i in 0..10 {
        vec.push(i as u8);
    }
    vec
}

#[profile]
pub fn arrayvec_push_10_u64() -> ArrayVec<u64, 10> {
    let mut vec: ArrayVec<u64, 10> = ArrayVec::new();
    for i in 0..10 {
        vec.push(i as u64);
    }
    vec
}

#[profile]
pub fn arrayvec_push_10_pubkey(program_id: &Pubkey) -> Result<ArrayVec<Pubkey, 10>, ProgramError> {
    let mut vec: ArrayVec<Pubkey, 10> = ArrayVec::new();
    for _ in 0..10 {
        vec.push(*program_id);
    }
    Ok(vec)
}
