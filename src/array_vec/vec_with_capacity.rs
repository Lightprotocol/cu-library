use light_program_profiler::profile;
use pinocchio::ProgramResult;
use arrayvec::ArrayVec;

#[profile]
pub fn array_vec_with_capacity_10() -> ProgramResult {
    let _vec: ArrayVec<u8, 10> = ArrayVec::new();
    Ok(())
}

#[profile]
pub fn array_vec_with_capacity_100() -> ProgramResult {
    let _vec: ArrayVec<u8, 100> = ArrayVec::new();
    Ok(())
}