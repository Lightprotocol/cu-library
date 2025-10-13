use std::ptr::read_unaligned;

use tinyvec::ArrayVec;
use light_program_profiler::profile;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

// Helper function to create a pre-populated ArrayVec - NOT profiled
pub fn create_populated_vec(_program_id: &Pubkey) -> ArrayVec<[Pubkey; 10]> {
    let mut vec: ArrayVec<[Pubkey; 10]> = ArrayVec::new();
    for i in 0..10 {
        vec.push(Pubkey::from([(i + 1) as u8; 32]));
    }
    vec
}

// Helper: Create vec with target at first position (1 iteration to find)
pub fn create_vec_target_first(target: &Pubkey, filler: &Pubkey) -> ArrayVec<[Pubkey; 10]> {
    let mut vec: ArrayVec<[Pubkey; 10]> = ArrayVec::new();
    vec.push(*target);
    for _ in 1..10 {
        vec.push(*filler);
    }
    vec
}

// Helper: Create vec with target at last position (10 iterations to find)
pub fn create_vec_target_last(target: &Pubkey, filler: &Pubkey) -> ArrayVec<[Pubkey; 10]> {
    let mut vec: ArrayVec<[Pubkey; 10]> = ArrayVec::new();
    for _ in 0..9 {
        vec.push(*filler);
    }
    vec.push(*target);
    vec
}

#[profile]
pub fn get_first_pubkey(vec: &ArrayVec<[Pubkey; 10]>) -> Result<&Pubkey, ProgramError> {
    vec.get(0).ok_or(ProgramError::InvalidAccountData)
}

#[profile]
pub fn get_10th_pubkey(vec: &ArrayVec<[Pubkey; 10]>) -> Result<&Pubkey, ProgramError> {
    vec.get(9).ok_or(ProgramError::InvalidAccountData)
}

#[profile]
pub fn find_pubkey_1_iters<'a>(
    vec: &'a ArrayVec<[Pubkey; 10]>,
    target: &Pubkey,
) -> Result<&'a Pubkey, ProgramError> {
    vec.iter()
        .find(|x| pubkey_eq(x, target))
        .ok_or(ProgramError::InvalidAccountData)
}

#[profile]
pub fn find_pubkey_10_iters<'a>(
    vec: &'a ArrayVec<[Pubkey; 10]>,
    target: &Pubkey,
) -> Result<&'a Pubkey, ProgramError> {
    vec.iter()
        .find(|x| pubkey_eq(x, target))
        .ok_or(ProgramError::InvalidAccountData)
}

#[profile]
pub fn position_pubkey_1_iters(vec: &ArrayVec<[Pubkey; 10]>, target: &Pubkey) -> Option<usize> {
    vec.iter().position(|x| pubkey_eq(x, target))
}

#[profile]
pub fn position_pubkey_10_iters(vec: &ArrayVec<[Pubkey; 10]>, target: &Pubkey) -> Option<usize> {
    vec.iter().position(|x| pubkey_eq(x, target))
}

#[profile]
pub fn update_index(vec: &mut ArrayVec<[Pubkey; 10]>, index: usize, new_value: &Pubkey) {
    vec[index] = *new_value;
}

#[profile]
pub fn update_get_mut(
    vec: &mut ArrayVec<[Pubkey; 10]>,
    index: usize,
    new_value: &Pubkey,
) -> Result<(), ProgramError> {
    let element = vec.get_mut(index).ok_or(ProgramError::InvalidAccountData)?;
    *element = *new_value;
    Ok(())
}

#[profile]
pub fn update_iter_mut_find(
    vec: &mut ArrayVec<[Pubkey; 10]>,
    target: &Pubkey,
    new_value: &Pubkey,
) -> Result<(), ProgramError> {
    let element = vec
        .iter_mut()
        .find(|x| pubkey_eq(x, target))
        .ok_or(ProgramError::InvalidAccountData)?;
    *element = *new_value;
    Ok(())
}
#[inline(always)]
pub const fn pubkey_eq(p1: &Pubkey, p2: &Pubkey) -> bool {
    let p1_ptr = p1.as_ptr() as *const u64;
    let p2_ptr = p2.as_ptr() as *const u64;

    unsafe {
        read_unaligned(p1_ptr) == read_unaligned(p2_ptr)
            && read_unaligned(p1_ptr.add(1)) == read_unaligned(p2_ptr.add(1))
            && read_unaligned(p1_ptr.add(2)) == read_unaligned(p2_ptr.add(2))
            && read_unaligned(p1_ptr.add(3)) == read_unaligned(p2_ptr.add(3))
    }
}
