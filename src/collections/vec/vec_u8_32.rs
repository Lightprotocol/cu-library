use light_program_profiler::profile;

#[profile]
pub fn vec_u8_32_index(vec: &Vec<u8>) -> &u8 {
    &vec[5]
}

#[profile]
pub fn vec_u8_32_get(vec: &Vec<u8>) -> Option<&u8> {
    vec.get(5)
}

#[profile]
pub fn vec_u8_32_get_ok_or<'a>(vec: &'a Vec<u8>) -> Result<&'a u8, &'static str> {
    vec.get(5).ok_or("Index out of bounds")
}

#[profile]
pub fn vec_u8_32_if_let_get(vec: &Vec<u8>) -> &u8 {
    if let Some(value) = vec.get(5) {
        value
    } else {
        panic!("Index out of bounds")
    }
}
