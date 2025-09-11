use light_program_profiler::profile;
use pinocchio::ProgramResult;
use arrayvec::ArrayVec;

#[profile]
pub fn array_vec_new() -> ProgramResult {
    let _vec: ArrayVec<u8, 32> = ArrayVec::new();
    Ok(())
}