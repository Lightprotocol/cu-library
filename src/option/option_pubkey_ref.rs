use light_program_profiler::profile;
use pinocchio::pubkey::Pubkey;

// Option handling with Pubkey reference and map(deref)

#[profile]
pub fn option_pubkey_ref_map_deref(pubkey_ref: Option<&Pubkey>) -> Option<Pubkey> {
    pubkey_ref.map(|p| *p)
}

#[profile]
pub fn option_pubkey_as_ref_map_convert(pubkey_bytes: Option<[u8; 32]>) -> Option<Pubkey> {
    pubkey_bytes.as_ref().map(|bytes| Pubkey::from(*bytes))
}