use core::mem::MaybeUninit;
use light_program_profiler::profile;
use tinyvec::ArrayVec;

// Helper functions to create test seed data - NOT profiled

pub fn create_1_seed() -> Vec<Vec<u8>> {
    vec![vec![1u8; 32]]
}

pub fn create_3_seeds() -> Vec<Vec<u8>> {
    vec![vec![1u8; 32], vec![2u8; 32], vec![3u8; 32]]
}

pub fn create_16_seeds() -> Vec<Vec<u8>> {
    let mut seeds = Vec::with_capacity(16);
    for i in 0..16 {
        seeds.push(vec![i as u8; 32]);
    }
    seeds
}

// Vec benchmarks - collecting into heap-allocated Vec

#[profile]
pub fn collect_vec_1_seed(seeds: &Vec<Vec<u8>>) -> Vec<&[u8]> {
    let seed_refs: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
    seed_refs
}

#[profile]
pub fn collect_vec_3_seeds(seeds: &Vec<Vec<u8>>) -> Vec<&[u8]> {
    let seed_refs: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
    seed_refs
}

#[profile]
pub fn collect_vec_16_seeds(seeds: &Vec<Vec<u8>>) -> Vec<&[u8]> {
    let seed_refs: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
    seed_refs
}

// ArrayVec benchmarks - collecting into stack-allocated ArrayVec

#[profile]
pub fn collect_arrayvec_1_seed(seeds: &Vec<Vec<u8>>) -> ArrayVec<[&[u8]; 1]> {
    let seed_refs: ArrayVec<[&[u8]; 1]> = seeds.iter().map(|s| s.as_slice()).collect();
    seed_refs
}

#[profile]
pub fn collect_arrayvec_3_seeds(seeds: &Vec<Vec<u8>>) -> ArrayVec<[&[u8]; 3]> {
    let seed_refs: ArrayVec<[&[u8]; 3]> = seeds.iter().map(|s| s.as_slice()).collect();
    seed_refs
}

#[profile]
pub fn collect_arrayvec_3_seeds_16_capacity(seeds: &Vec<Vec<u8>>) -> ArrayVec<[&[u8]; 16]> {
    let seed_refs: ArrayVec<[&[u8]; 16]> = seeds.iter().map(|s| s.as_slice()).collect();
    seed_refs
}

#[profile]
pub fn collect_arrayvec_16_seeds(seeds: &Vec<Vec<u8>>) -> ArrayVec<[&[u8]; 16]> {
    let seed_refs: ArrayVec<[&[u8]; 16]> = seeds.iter().map(|s| s.as_slice()).collect();
    seed_refs
}

#[profile]
pub fn array_3_seeds(seeds: &Vec<Vec<u8>>) -> [&[u8]; 3] {
    [
        seeds[0].as_slice(),
        seeds[1].as_slice(),
        seeds[2].as_slice(),
    ]
}

#[profile]
pub fn array_3_seeds_maybeuninit(seeds: &Vec<Vec<u8>>) -> [&[u8]; 3] {
    unsafe {
        let mut arr: [MaybeUninit<&[u8]>; 3] = [
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        ];
        arr[0].write(seeds.get_unchecked(0).as_slice());
        arr[1].write(seeds.get_unchecked(1).as_slice());
        arr[2].write(seeds.get_unchecked(2).as_slice());
        core::mem::transmute::<[MaybeUninit<&[u8]>; 3], [&[u8]; 3]>(arr)
    }
}

#[profile]
pub fn array_3_seeds_maybeuninit_no_transmute(seeds: &Vec<Vec<u8>>) -> [MaybeUninit<&[u8]>; 3] {
    unsafe {
        let mut arr: [MaybeUninit<&[u8]>; 3] = [
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        ];
        arr[0].write(seeds.get_unchecked(0).as_slice());
        arr[1].write(seeds.get_unchecked(1).as_slice());
        arr[2].write(seeds.get_unchecked(2).as_slice());
        arr
    }
}

#[profile]
pub fn array_3_seeds_ptr(seeds: &Vec<Vec<u8>>) -> [&[u8]; 3] {
    unsafe {
        let ptr = seeds.as_ptr();
        [
            (*ptr).as_slice(),
            (*ptr.add(1)).as_slice(),
            (*ptr.add(2)).as_slice(),
        ]
    }
}
