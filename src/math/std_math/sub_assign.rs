use light_program_profiler::profile;

#[profile]
pub fn sub_assign_u8() -> u8 {
    let mut a: u8 = 100;
    let b: u8 = 50;
    a -= b;
    a
}

#[profile]
pub fn sub_assign_u16() -> u16 {
    let mut a: u16 = 10000;
    let b: u16 = 5000;
    a -= b;
    a
}

#[profile]
pub fn sub_assign_u32() -> u32 {
    let mut a: u32 = 1000000;
    let b: u32 = 500000;
    a -= b;
    a
}

#[profile]
pub fn sub_assign_u64() -> u64 {
    let mut a: u64 = 100000000;
    let b: u64 = 50000000;
    a -= b;
    a
}

#[profile]
pub fn sub_assign_u128() -> u128 {
    let mut a: u128 = 10000000000000000;
    let b: u128 = 5000000000000000;
    a -= b;
    a
}