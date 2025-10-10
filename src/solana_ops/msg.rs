use light_program_profiler::profile;
use pinocchio::ProgramResult;

#[profile]
pub fn msg10_chars() -> ProgramResult {
    solana_msg::msg!("0123456789");
    Ok(())
}
