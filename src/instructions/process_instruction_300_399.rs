use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::instructions::discriminator::CuLibraryInstruction;
use crate::collections;

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
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
