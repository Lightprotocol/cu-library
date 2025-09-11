use light_program_profiler::profile;

#[profile]
pub fn checked_mul_u8() -> Option<u8> {
    let a: u8 = 10;
    let b: u8 = 5;
    a.checked_mul(b)
}

#[profile]
pub fn checked_mul_u16() -> Option<u16> {
    let a: u16 = 100;
    let b: u16 = 50;
    a.checked_mul(b)
}

#[profile]
pub fn checked_mul_u32() -> Option<u32> {
    let a: u32 = 10000;
    let b: u32 = 500;
    a.checked_mul(b)
}

#[profile]
pub fn checked_mul_u64() -> Option<u64> {
    let a: u64 = 1000000;
    let b: u64 = 500;
    a.checked_mul(b)
}

#[profile]
pub fn checked_mul_u128() -> Option<u128> {
    let a: u128 = 100000000;
    let b: u128 = 50000;
    a.checked_mul(b)
}