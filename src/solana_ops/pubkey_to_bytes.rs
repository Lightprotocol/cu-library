use light_program_profiler::profile;

// Solana Pubkey to_bytes operation

#[profile]
pub fn solana_pubkey_to_bytes(pubkey: &solana_pubkey::Pubkey) -> [u8; 32] {
    pubkey.to_bytes()
}