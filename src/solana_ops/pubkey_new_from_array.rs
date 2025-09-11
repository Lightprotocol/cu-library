use light_program_profiler::profile;
use pinocchio::{ProgramResult, pubkey::Pubkey};

#[profile]
pub fn solana_pubkey_new_from_array(program_id: &Pubkey) -> ProgramResult {
    let _id = solana_pubkey::Pubkey::new_from_array(*program_id);
    Ok(())
}