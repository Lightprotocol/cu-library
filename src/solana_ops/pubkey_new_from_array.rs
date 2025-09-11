use light_program_profiler::profile;
use pinocchio::pubkey::Pubkey;

#[profile]
pub fn solana_pubkey_new_from_array(program_id: &Pubkey) -> solana_pubkey::Pubkey {
    solana_pubkey::Pubkey::new_from_array(*program_id)
}