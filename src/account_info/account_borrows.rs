use light_program_profiler::profile;
use pinocchio::{account_info::{AccountInfo, BorrowState}, program_error::ProgramError};

#[profile]
pub fn is_borrowed(account: &AccountInfo) -> bool {
    account.is_borrowed(BorrowState::Borrowed)
}

#[profile]
pub fn borrow_lamports_unchecked(account: &AccountInfo) -> &u64 {
    unsafe { account.borrow_lamports_unchecked() }
}

#[profile]
#[allow(clippy::mut_from_ref)]
pub fn borrow_mut_lamports_unchecked(account: &AccountInfo) -> &mut u64 {
    unsafe { account.borrow_mut_lamports_unchecked() }
}

#[profile]
pub fn borrow_data_unchecked(account: &AccountInfo) -> &[u8] {
    unsafe { account.borrow_data_unchecked() }
}

#[profile]
#[allow(clippy::mut_from_ref)]
pub fn borrow_mut_data_unchecked(account: &AccountInfo) -> &mut [u8] {
    unsafe { account.borrow_mut_data_unchecked() }
}

#[profile]
pub fn try_borrow_lamports(account: &AccountInfo) -> Result<(), ProgramError> {
    account.try_borrow_lamports()?;
    Ok(())
}

#[profile]
pub fn try_borrow_mut_lamports(account: &AccountInfo) -> Result<(), ProgramError> {
    account.try_borrow_mut_lamports()?;
    Ok(())
}

#[profile]
pub fn can_borrow_lamports(account: &AccountInfo) -> Result<(), ProgramError> {
    account.can_borrow_lamports()
}

#[profile]
pub fn can_borrow_mut_lamports(account: &AccountInfo) -> Result<(), ProgramError> {
    account.can_borrow_mut_lamports()
}

#[profile]
pub fn try_borrow_data(account: &AccountInfo) -> Result<(), ProgramError> {
    account.try_borrow_data()?;
    Ok(())
}

#[profile]
pub fn try_borrow_mut_data(account: &AccountInfo) -> Result<(), ProgramError> {
    account.try_borrow_mut_data()?;
    Ok(())
}

#[profile]
pub fn can_borrow_data(account: &AccountInfo) -> Result<(), ProgramError> {
    account.can_borrow_data()
}

#[profile]
pub fn can_borrow_mut_data(account: &AccountInfo) -> Result<(), ProgramError> {
    account.can_borrow_mut_data()
}
