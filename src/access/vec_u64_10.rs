use light_program_profiler::profile;

#[profile]
pub fn access_vec_u64_10_index(vec: &Vec<u64>) -> &u64 {
    &vec[3]
}

#[profile]
pub fn access_vec_u64_10_get(vec: &Vec<u64>) -> Option<&u64> {
    vec.get(3)
}

#[profile]
pub fn access_vec_u64_10_get_ok_or<'a>(vec: &'a Vec<u64>) -> Result<&'a u64, &'static str> {
    vec.get(3).ok_or("Index out of bounds")
}

#[profile]
pub fn access_vec_u64_10_if_let_get(vec: &Vec<u64>) -> &u64 {
    if let Some(value) = vec.get(3) {
        value
    } else {
        panic!("Index out of bounds")
    }
}
