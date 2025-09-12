use light_program_profiler::profile;

#[profile]
pub fn access_array_u8_32_index(array: &[u8; 32]) -> &u8 {
    &array[5]
}

#[profile]
pub fn access_array_u8_32_get(array: &[u8; 32]) -> Option<&u8> {
    array.get(5)
}

#[profile]
pub fn access_array_u8_32_get_ok_or<'a>(array: &'a [u8; 32]) -> Result<&'a u8, &'static str> {
    array.get(5).ok_or("Index out of bounds")
}

#[profile]
pub fn access_array_u8_32_if_let_get(array: &[u8; 32]) -> &u8 {
    if let Some(value) = array.get(5) {
        value
    } else {
        panic!("Index out of bounds")
    }
}