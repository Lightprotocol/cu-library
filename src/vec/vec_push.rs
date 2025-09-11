use light_program_profiler::profile;
use pinocchio::pubkey::Pubkey;

#[profile]
pub fn vec_push_u8() -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    vec.push(42u8);
    vec
}

#[profile]
pub fn vec_push_u64() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    vec.push(12345678u64);
    vec
}

#[profile]
pub fn vec_push_pubkey(program_id: &Pubkey) -> Vec<Pubkey> {
    let mut vec: Vec<Pubkey> = Vec::new();
    vec.push(*program_id);
    vec
}

#[profile]
pub fn vec_push_10_u8() -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..10 {
        vec.push(i as u8);
    }
    vec
}

#[profile]
pub fn vec_push_10_u64() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    for i in 0..10 {
        vec.push(i as u64);
    }
    vec
}

#[profile]
pub fn vec_push_10_pubkey(program_id: &Pubkey) -> Vec<Pubkey> {
    let mut vec: Vec<Pubkey> = Vec::new();
    for _ in 0..10 {
        vec.push(*program_id);
    }
    vec
}

#[profile]
pub fn vec_push_u8_with_capacity() -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(1);
    vec.push(42u8);
    vec
}

#[profile]
pub fn vec_push_u64_with_capacity() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(1);
    vec.push(12345678u64);
    vec
}

#[profile]
pub fn vec_push_pubkey_with_capacity(program_id: &Pubkey) -> Vec<Pubkey> {
    let mut vec: Vec<Pubkey> = Vec::with_capacity(1);
    vec.push(*program_id);
    vec
}

#[profile]
pub fn vec_push_10_u8_with_capacity() -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(i as u8);
    }
    vec
}

#[profile]
pub fn vec_push_10_u64_with_capacity() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(i as u64);
    }
    vec
}

#[profile]
pub fn vec_push_10_pubkey_with_capacity(program_id: &Pubkey) -> Vec<Pubkey> {
    let mut vec: Vec<Pubkey> = Vec::with_capacity(10);
    for _ in 0..10 {
        vec.push(*program_id);
    }
    vec
}
