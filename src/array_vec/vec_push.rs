use light_program_profiler::profile;
use pinocchio::{ProgramResult, pubkey::Pubkey};
use arrayvec::ArrayVec;

#[profile]
pub fn array_vec_push_u8() -> ProgramResult {
    let mut vec: ArrayVec<u8, 10> = ArrayVec::new();
    vec.push(42u8);
    Ok(())
}

#[profile]
pub fn array_vec_push_u64() -> ProgramResult {
    let mut vec: ArrayVec<u64, 10> = ArrayVec::new();
    vec.push(12345678u64);
    Ok(())
}

#[profile]
pub fn array_vec_push_pubkey(program_id: &Pubkey) -> ProgramResult {
    let mut vec: ArrayVec<Pubkey, 10> = ArrayVec::new();
    vec.push(*program_id);
    Ok(())
}