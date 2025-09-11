use light_program_profiler::profile;
use pinocchio::{msg, ProgramResult};

#[profile]
pub fn pinocchio_msg10_chars() -> ProgramResult {
    msg!("0123456789");
    Ok(())
}
