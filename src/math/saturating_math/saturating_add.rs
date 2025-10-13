use light_program_profiler::profile;

#[profile]
pub fn add_u8() -> u8 {
    let a: u8 = 250;
    let b: u8 = 50;
    a.saturating_add(b)
}

#[profile]
pub fn add_u16() -> u16 {
    let a: u16 = 65000;
    let b: u16 = 5000;
    a.saturating_add(b)
}

#[profile]
pub fn add_u32() -> u32 {
    let a: u32 = 4000000000;
    let b: u32 = 500000000;
    a.saturating_add(b)
}

#[profile]
pub fn add_u64() -> u64 {
    let a: u64 = 18000000000000000000;
    let b: u64 = 500000000000000000;
    a.saturating_add(b)
}

#[profile]
pub fn add_u128() -> u128 {
    let a: u128 = 340000000000000000000000000000000000000;
    let b: u128 = 5000000000000000000000000000000000000;
    a.saturating_add(b)
}