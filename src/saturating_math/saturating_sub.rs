use light_program_profiler::profile;

#[profile]
pub fn saturating_sub_u8() -> u8 {
    let a: u8 = 50;
    let b: u8 = 100;
    a.saturating_sub(b)
}

#[profile]
pub fn saturating_sub_u16() -> u16 {
    let a: u16 = 5000;
    let b: u16 = 10000;
    a.saturating_sub(b)
}

#[profile]
pub fn saturating_sub_u32() -> u32 {
    let a: u32 = 500000;
    let b: u32 = 1000000;
    a.saturating_sub(b)
}

#[profile]
pub fn saturating_sub_u64() -> u64 {
    let a: u64 = 50000000;
    let b: u64 = 100000000;
    a.saturating_sub(b)
}

#[profile]
pub fn saturating_sub_u128() -> u128 {
    let a: u128 = 5000000000000000;
    let b: u128 = 10000000000000000;
    a.saturating_sub(b)
}