use light_program_profiler::profile;

#[profile]
pub fn mul_u8() -> u8 {
    let a: u8 = 10;
    let b: u8 = 5;
    a * b
}

#[profile]
pub fn mul_u16() -> u16 {
    let a: u16 = 100;
    let b: u16 = 50;
    a * b
}

#[profile]
pub fn mul_u32() -> u32 {
    let a: u32 = 10000;
    let b: u32 = 500;
    a * b
}

#[profile]
pub fn mul_u64() -> u64 {
    let a: u64 = 1000000;
    let b: u64 = 500;
    a * b
}

#[profile]
pub fn mul_u128() -> u128 {
    let a: u128 = 100000000;
    let b: u128 = 50000;
    a * b
}