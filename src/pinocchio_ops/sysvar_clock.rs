use light_program_profiler::profile;
use pinocchio::{sysvars::{clock::Clock, Sysvar}, ProgramResult};

#[profile]
pub fn pinocchio_clock_get_slot() -> ProgramResult {
    let clock = Clock::get()?;
    let _slot = clock.slot;
    Ok(())
}