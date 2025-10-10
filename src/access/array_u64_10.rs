use light_program_profiler::profile;

#[profile]
pub fn array_u64_10_index(array: &[u64; 10]) -> &u64 {
    &array[3]
}

#[profile]
pub fn array_u64_10_get(array: &[u64; 10]) -> Option<&u64> {
    array.get(3)
}

#[profile]
pub fn array_u64_10_get_ok_or<'a>(array: &'a [u64; 10]) -> Result<&'a u64, &'static str> {
    array.get(3).ok_or("Index out of bounds")
}

#[profile]
pub fn array_u64_10_if_let_get(array: &[u64; 10]) -> &u64 {
    if let Some(value) = array.get(3) {
        value
    } else {
        panic!("Index out of bounds")
    }
}