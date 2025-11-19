use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::instructions::discriminator::CuLibraryInstruction;
use crate::collections;
use crate::solana_crates;

#[inline(never)]
pub fn process_instruction_300_399(
    instruction: CuLibraryInstruction,
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> Result<(), ProgramError> {
    match instruction {
        // Tinyvec heap benchmarks (TinyVec with alloc feature)
        CuLibraryInstruction::TinyvecHeapU8New => {
            let res = collections::tinyvec_heap::vec_new::u8_new();
            solana_msg::msg!("vec len: {}", res.len());
        }
        CuLibraryInstruction::TinyvecHeapPushU8 => {
            let mut vec = collections::tinyvec_heap::vec_push::create_empty_u8_vec();
            collections::tinyvec_heap::vec_push::push_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecHeapPushU64 => {
            let mut vec = collections::tinyvec_heap::vec_push::create_empty_u64_vec();
            collections::tinyvec_heap::vec_push::push_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecHeapPushPubkey => {
            let mut vec = collections::tinyvec_heap::vec_push::create_empty_pubkey_vec();
            collections::tinyvec_heap::vec_push::push_pubkey(&mut vec, program_id);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecHeapPush10U8 => {
            let mut vec = collections::tinyvec_heap::vec_push::create_empty_u8_vec();
            collections::tinyvec_heap::vec_push::push_10_u8(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecHeapPush10U64 => {
            let mut vec = collections::tinyvec_heap::vec_push::create_empty_u64_vec();
            collections::tinyvec_heap::vec_push::push_10_u64(&mut vec);
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        CuLibraryInstruction::TinyvecHeapPush10Pubkey => {
            let mut vec = collections::tinyvec_heap::vec_push::create_empty_pubkey_vec();
            collections::tinyvec_heap::vec_push::push_10_pubkey(&mut vec, program_id)?;
            solana_msg::msg!("vec: {:?}", vec.as_slice());
        }
        // Seed reference collection benchmarks
        CuLibraryInstruction::CollectVec1Seed => {
            let seeds = solana_crates::seed_references::create_1_seed();
            let res = solana_crates::seed_references::collect_vec_1_seed(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_slice());
        }
        CuLibraryInstruction::CollectVec3Seeds => {
            let seeds = solana_crates::seed_references::create_3_seeds();
            let res = solana_crates::seed_references::collect_vec_3_seeds(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_slice());
        }
        CuLibraryInstruction::CollectVec16Seeds => {
            let seeds = solana_crates::seed_references::create_16_seeds();
            let res = solana_crates::seed_references::collect_vec_16_seeds(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_slice());
        }
        CuLibraryInstruction::CollectArrayVec1Seed => {
            let seeds = solana_crates::seed_references::create_1_seed();
            let res = solana_crates::seed_references::collect_arrayvec_1_seed(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_slice());
        }
        CuLibraryInstruction::CollectArrayVec3Seeds => {
            let seeds = solana_crates::seed_references::create_3_seeds();
            let res = solana_crates::seed_references::collect_arrayvec_3_seeds(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_slice());
        }
        CuLibraryInstruction::CollectArrayVec16Seeds => {
            let seeds = solana_crates::seed_references::create_16_seeds();
            let res = solana_crates::seed_references::collect_arrayvec_16_seeds(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_slice());
        }
        CuLibraryInstruction::CollectArrayVec3Seeds16Capacity => {
            let seeds = solana_crates::seed_references::create_3_seeds();
            let res = solana_crates::seed_references::collect_arrayvec_3_seeds_16_capacity(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_slice());
        }
        CuLibraryInstruction::Array3Seeds => {
            let seeds = solana_crates::seed_references::create_3_seeds();
            let res = solana_crates::seed_references::array_3_seeds(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_ref());
        }
        CuLibraryInstruction::Array3SeedsMaybeUninit => {
            let seeds = solana_crates::seed_references::create_3_seeds();
            let res = solana_crates::seed_references::array_3_seeds_maybeuninit(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_ref());
        }
        CuLibraryInstruction::Array3SeedsMaybeUninitNoTransmute => {
            let seeds = solana_crates::seed_references::create_3_seeds();
            let res = solana_crates::seed_references::array_3_seeds_maybeuninit_no_transmute(&seeds);
            unsafe {
                solana_msg::msg!(
                    "seed_refs: {:?}",
                    [res[0].assume_init(), res[1].assume_init(), res[2].assume_init()].as_ref()
                );
            }
        }
        CuLibraryInstruction::Array3SeedsPtr => {
            let seeds = solana_crates::seed_references::create_3_seeds();
            let res = solana_crates::seed_references::array_3_seeds_ptr(&seeds);
            solana_msg::msg!("seed_refs: {:?}", res.as_ref());
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
