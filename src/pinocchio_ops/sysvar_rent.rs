use light_program_profiler::profile;
use pinocchio::{ProgramResult, sysvars::{rent::Rent, Sysvar}};

#[profile]
pub fn pinocchio_sysvar_rent_exemption_165() -> ProgramResult {
    let rent = Rent::get()?;
    let _exemption = rent.minimum_balance(165);
    Ok(())
}