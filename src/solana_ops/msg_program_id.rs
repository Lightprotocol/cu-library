use light_program_profiler::profile;
use pinocchio::{ProgramResult, pubkey::Pubkey};

#[profile]
pub fn solana_msg_program_id(program_id: &Pubkey) -> ProgramResult {
    solana_msg::msg!("Program ID: {:?}", program_id);
    Ok(())
}