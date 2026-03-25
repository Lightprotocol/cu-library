use std::collections::HashMap;

use cu_library::CuLibraryInstruction;
use light_program_profiler::mollusk::{
    extract_category_and_file, register_profiling_syscalls, take_profiling_results,
    write_categorized_readme, BenchmarkEntry, BenchmarkResults, ReadmeConfig,
};
use mollusk_svm::Mollusk;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

#[test]
fn bench_cu_operations() {
    let program_id = Pubkey::new_unique();

    let mut mollusk = Mollusk::default();
    register_profiling_syscalls(&mut mollusk);
    mollusk.add_program(
        &program_id,
        "cu_library",
        &mollusk_svm::program::loader_keys::LOADER_V3,
    );

    let payer = Pubkey::new_unique();
    let test_account_pubkey = Pubkey::new_unique();
    let cpi_pubkeys: Vec<Pubkey> = (0..10).map(|_| Pubkey::new_unique()).collect();

    let payer_account = Account {
        lamports: 10_000_000_000,
        data: vec![],
        owner: Pubkey::default(),
        executable: false,
        rent_epoch: 0,
    };

    let mut results_by_category: BenchmarkResults = BenchmarkResults::new();

    let instructions = vec![
        CuLibraryInstruction::Baseline,
        CuLibraryInstruction::Msg10,
        CuLibraryInstruction::SolanaMsg10,
        CuLibraryInstruction::SolanaMsgProgramId,
        CuLibraryInstruction::SolanaPubkeyNewFromArray,
        CuLibraryInstruction::SolanaPubkeyToBytes,
        CuLibraryInstruction::PinocchioSysvarRentExemption165,
        CuLibraryInstruction::PinocchioClockGetSlot,
        CuLibraryInstruction::ArrayvecNew,
        CuLibraryInstruction::ArrayvecPushPubkey,
        CuLibraryInstruction::ArrayvecPushU64,
        CuLibraryInstruction::ArrayvecPushU8,
        CuLibraryInstruction::ArrayvecWithCapacity10,
        CuLibraryInstruction::ArrayvecWithCapacity100,
        CuLibraryInstruction::ArrayvecPush10U8,
        CuLibraryInstruction::ArrayvecPush10U64,
        CuLibraryInstruction::ArrayvecPush10Pubkey,
        CuLibraryInstruction::ArrayvecGetFirstPubkey,
        CuLibraryInstruction::ArrayvecGet10thPubkey,
        CuLibraryInstruction::ArrayvecFindPubkey1Iters,
        CuLibraryInstruction::ArrayvecFindPubkey10Iters,
        CuLibraryInstruction::ArrayvecPositionPubkey1Iters,
        CuLibraryInstruction::ArrayvecPositionPubkey10Iters,
        CuLibraryInstruction::ArrayvecUpdateIndex,
        CuLibraryInstruction::ArrayvecUpdateGetMut,
        CuLibraryInstruction::ArrayvecUpdateIterMutFind,
        // Tinyvec benchmarks
        CuLibraryInstruction::TinyvecU8New,
        CuLibraryInstruction::TinyvecU8WithCapacity10,
        CuLibraryInstruction::TinyvecU8WithCapacity100,
        CuLibraryInstruction::TinyvecPushU8,
        CuLibraryInstruction::TinyvecPushU64,
        CuLibraryInstruction::TinyvecPushPubkey,
        CuLibraryInstruction::TinyvecPush10U8,
        CuLibraryInstruction::TinyvecPush10U64,
        CuLibraryInstruction::TinyvecPush10Pubkey,
        CuLibraryInstruction::TinyvecGetFirstPubkey,
        CuLibraryInstruction::TinyvecGet10thPubkey,
        CuLibraryInstruction::TinyvecFindPubkey1Iters,
        CuLibraryInstruction::TinyvecFindPubkey10Iters,
        CuLibraryInstruction::TinyvecPositionPubkey1Iters,
        CuLibraryInstruction::TinyvecPositionPubkey10Iters,
        CuLibraryInstruction::TinyvecUpdateIndex,
        CuLibraryInstruction::TinyvecUpdateGetMut,
        CuLibraryInstruction::TinyvecUpdateIterMutFind,
        // Tinyvec heap benchmarks (TinyVec with alloc feature)
        CuLibraryInstruction::TinyvecHeapU8New,
        CuLibraryInstruction::TinyvecHeapPushU8,
        CuLibraryInstruction::TinyvecHeapPushU64,
        CuLibraryInstruction::TinyvecHeapPushPubkey,
        CuLibraryInstruction::TinyvecHeapPush10U8,
        CuLibraryInstruction::TinyvecHeapPush10U64,
        CuLibraryInstruction::TinyvecHeapPush10Pubkey,
        // Heapless benchmarks
        CuLibraryInstruction::HeaplessU8New,
        CuLibraryInstruction::HeaplessU8WithCapacity10,
        CuLibraryInstruction::HeaplessU8WithCapacity100,
        CuLibraryInstruction::HeaplessPushU8,
        CuLibraryInstruction::HeaplessPushU64,
        CuLibraryInstruction::HeaplessPushPubkey,
        CuLibraryInstruction::HeaplessPush10U8,
        CuLibraryInstruction::HeaplessPush10U64,
        CuLibraryInstruction::HeaplessPush10Pubkey,
        CuLibraryInstruction::HeaplessGetFirstPubkey,
        CuLibraryInstruction::HeaplessGet10thPubkey,
        CuLibraryInstruction::HeaplessFindPubkey1Iters,
        CuLibraryInstruction::HeaplessFindPubkey10Iters,
        CuLibraryInstruction::HeaplessPositionPubkey1Iters,
        CuLibraryInstruction::HeaplessPositionPubkey10Iters,
        CuLibraryInstruction::HeaplessUpdateIndex,
        CuLibraryInstruction::HeaplessUpdateGetMut,
        CuLibraryInstruction::HeaplessUpdateIterMutFind,
        // Smallvec benchmarks
        CuLibraryInstruction::SmallvecU8New,
        CuLibraryInstruction::SmallvecU8WithCapacity10,
        CuLibraryInstruction::SmallvecU8WithCapacity128,
        CuLibraryInstruction::SmallvecPushU8,
        CuLibraryInstruction::SmallvecPushU64,
        CuLibraryInstruction::SmallvecPushPubkey,
        CuLibraryInstruction::SmallvecPush10U8,
        CuLibraryInstruction::SmallvecPush10U64,
        CuLibraryInstruction::SmallvecPush10Pubkey,
        CuLibraryInstruction::SmallvecGetFirstPubkey,
        CuLibraryInstruction::SmallvecGet10thPubkey,
        CuLibraryInstruction::SmallvecFindPubkey1Iters,
        CuLibraryInstruction::SmallvecFindPubkey10Iters,
        CuLibraryInstruction::SmallvecPositionPubkey1Iters,
        CuLibraryInstruction::SmallvecPositionPubkey10Iters,
        CuLibraryInstruction::SmallvecUpdateIndex,
        CuLibraryInstruction::SmallvecUpdateGetMut,
        CuLibraryInstruction::SmallvecUpdateIterMutFind,
        CuLibraryInstruction::VecNew,
        CuLibraryInstruction::VecWithCapacity10,
        CuLibraryInstruction::VecWithCapacity100,
        CuLibraryInstruction::VecPushU8,
        CuLibraryInstruction::VecPushU64,
        CuLibraryInstruction::VecPushPubkey,
        CuLibraryInstruction::VecPush10U8,
        CuLibraryInstruction::VecPush10U64,
        CuLibraryInstruction::VecPush10Pubkey,
        CuLibraryInstruction::ArrayNew,
        CuLibraryInstruction::ArrayWithCapacity10,
        CuLibraryInstruction::ArrayWithCapacity100,
        CuLibraryInstruction::ArrayAssignU8,
        CuLibraryInstruction::ArrayAssignU64,
        CuLibraryInstruction::ArrayAssignPubkey,
        CuLibraryInstruction::ArrayAssign10U8,
        CuLibraryInstruction::ArrayAssign10U64,
        CuLibraryInstruction::ArrayAssign10Pubkey,
        CuLibraryInstruction::CheckedAddU8,
        CuLibraryInstruction::CheckedAddU16,
        CuLibraryInstruction::CheckedAddU32,
        CuLibraryInstruction::CheckedAddU64,
        CuLibraryInstruction::CheckedAddU128,
        CuLibraryInstruction::CheckedSubU8,
        CuLibraryInstruction::CheckedSubU16,
        CuLibraryInstruction::CheckedSubU32,
        CuLibraryInstruction::CheckedSubU64,
        CuLibraryInstruction::CheckedSubU128,
        CuLibraryInstruction::CheckedMulU8,
        CuLibraryInstruction::CheckedMulU16,
        CuLibraryInstruction::CheckedMulU32,
        CuLibraryInstruction::CheckedMulU64,
        CuLibraryInstruction::CheckedMulU128,
        CuLibraryInstruction::CheckedDivU8,
        CuLibraryInstruction::CheckedDivU16,
        CuLibraryInstruction::CheckedDivU32,
        CuLibraryInstruction::CheckedDivU64,
        CuLibraryInstruction::CheckedDivU128,
        CuLibraryInstruction::SaturatingAddU8,
        CuLibraryInstruction::SaturatingAddU16,
        CuLibraryInstruction::SaturatingAddU32,
        CuLibraryInstruction::SaturatingAddU64,
        CuLibraryInstruction::SaturatingAddU128,
        CuLibraryInstruction::SaturatingSubU8,
        CuLibraryInstruction::SaturatingSubU16,
        CuLibraryInstruction::SaturatingSubU32,
        CuLibraryInstruction::SaturatingSubU64,
        CuLibraryInstruction::SaturatingSubU128,
        CuLibraryInstruction::SaturatingMulU8,
        CuLibraryInstruction::SaturatingMulU16,
        CuLibraryInstruction::SaturatingMulU32,
        CuLibraryInstruction::SaturatingMulU64,
        CuLibraryInstruction::SaturatingMulU128,
        CuLibraryInstruction::StdAddU8,
        CuLibraryInstruction::StdAddU16,
        CuLibraryInstruction::StdAddU32,
        CuLibraryInstruction::StdAddU64,
        CuLibraryInstruction::StdAddU128,
        CuLibraryInstruction::StdSubU8,
        CuLibraryInstruction::StdSubU16,
        CuLibraryInstruction::StdSubU32,
        CuLibraryInstruction::StdSubU64,
        CuLibraryInstruction::StdSubU128,
        CuLibraryInstruction::StdMulU8,
        CuLibraryInstruction::StdMulU16,
        CuLibraryInstruction::StdMulU32,
        CuLibraryInstruction::StdMulU64,
        CuLibraryInstruction::StdMulU128,
        CuLibraryInstruction::StdDivU8,
        CuLibraryInstruction::StdDivU16,
        CuLibraryInstruction::StdDivU32,
        CuLibraryInstruction::StdDivU64,
        CuLibraryInstruction::StdDivU128,
        CuLibraryInstruction::AddAssignU8,
        CuLibraryInstruction::AddAssignU16,
        CuLibraryInstruction::AddAssignU32,
        CuLibraryInstruction::AddAssignU64,
        CuLibraryInstruction::AddAssignU128,
        CuLibraryInstruction::SubAssignU8,
        CuLibraryInstruction::SubAssignU16,
        CuLibraryInstruction::SubAssignU32,
        CuLibraryInstruction::SubAssignU64,
        CuLibraryInstruction::SubAssignU128,
        CuLibraryInstruction::VecPushU8WithCapacity,
        CuLibraryInstruction::VecPushU64WithCapacity,
        CuLibraryInstruction::VecPushPubkeyWithCapacity,
        CuLibraryInstruction::VecPush10U8WithCapacity,
        CuLibraryInstruction::VecPush10U64WithCapacity,
        CuLibraryInstruction::VecPush10PubkeyWithCapacity,
        CuLibraryInstruction::AccountInfoKey,
        CuLibraryInstruction::AccountInfoOwner,
        CuLibraryInstruction::AccountInfoIsSigner,
        CuLibraryInstruction::AccountInfoIsWritable,
        CuLibraryInstruction::AccountInfoExecutable,
        CuLibraryInstruction::AccountInfoDataLen,
        CuLibraryInstruction::AccountInfoLamports,
        CuLibraryInstruction::AccountInfoDataIsEmpty,
        CuLibraryInstruction::AccountInfoIsOwnedBy,
        CuLibraryInstruction::AccountInfoAssign,
        CuLibraryInstruction::AccountInfoIsBorrowed,
        CuLibraryInstruction::AccountInfoBorrowLamportsUnchecked,
        CuLibraryInstruction::AccountInfoBorrowMutLamportsUnchecked,
        CuLibraryInstruction::AccountInfoBorrowDataUnchecked,
        CuLibraryInstruction::AccountInfoBorrowMutDataUnchecked,
        CuLibraryInstruction::AccountInfoTryBorrowLamports,
        CuLibraryInstruction::AccountInfoTryBorrowMutLamports,
        CuLibraryInstruction::AccountInfoCanBorrowLamports,
        CuLibraryInstruction::AccountInfoCanBorrowMutLamports,
        CuLibraryInstruction::AccountInfoTryBorrowData,
        CuLibraryInstruction::AccountInfoTryBorrowMutData,
        CuLibraryInstruction::AccountInfoCanBorrowData,
        CuLibraryInstruction::AccountInfoCanBorrowMutData,
        CuLibraryInstruction::AccountInfoRealloc,
        // Close operations are skipped as they would affect subsequent tests
        // CuLibraryInstruction::AccountInfoClose,
        // CuLibraryInstruction::AccountInfoCloseUnchecked,
        CuLibraryInstruction::CpiAccountMetaArray10,
        CuLibraryInstruction::CpiAccountInfoArray10Ref,
        CuLibraryInstruction::CpiAccountInfoArray10Clone,
        CuLibraryInstruction::CpiAccountInfoArray10Move,
        CuLibraryInstruction::CpiArrayvecPushAccountMeta10,
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Ref,
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Clone,
        CuLibraryInstruction::CpiArrayvecPushAccountInfo10Move,
        CuLibraryInstruction::CpiAccountMetaArray10Loop,
        CuLibraryInstruction::CpiAccountInfoArray10RefLoop,
        CuLibraryInstruction::CpiAccountInfoArray10CloneLoop,
        CuLibraryInstruction::CpiAccountInfoArray10MoveLoop,
        CuLibraryInstruction::PartialEqU8,
        CuLibraryInstruction::PartialEqU16,
        CuLibraryInstruction::PartialEqU32,
        CuLibraryInstruction::PartialEqU64,
        CuLibraryInstruction::PartialEqU128,
        CuLibraryInstruction::PartialEqArrayU8_32Ref,
        CuLibraryInstruction::PartialEqArrayU8_32,
        CuLibraryInstruction::PartialEqArrayU16_32,
        CuLibraryInstruction::PartialEqArrayU32_32,
        CuLibraryInstruction::PartialEqArrayU64_32,
        CuLibraryInstruction::PartialEqU8Neq,
        CuLibraryInstruction::PartialEqU16Neq,
        CuLibraryInstruction::PartialEqU32Neq,
        CuLibraryInstruction::PartialEqU64Neq,
        CuLibraryInstruction::PartialEqU128Neq,
        CuLibraryInstruction::PartialEqArrayU8_32NeqRef,
        CuLibraryInstruction::PartialEqArrayU8_32Neq,
        CuLibraryInstruction::PartialEqArrayU8_32NeqDeref,
        CuLibraryInstruction::PartialEqArrayU16_32Neq,
        CuLibraryInstruction::PartialEqArrayU32_32Neq,
        CuLibraryInstruction::PartialEqArrayU64_32Neq,
        // Conversions
        CuLibraryInstruction::ConversionsSliceToArray32Unwrap,
        CuLibraryInstruction::ConversionsSliceToArray32MapErr,
        CuLibraryInstruction::ConversionsUsizeToU64Unwrap,
        CuLibraryInstruction::ConversionsUsizeToU64MapErr,
        CuLibraryInstruction::ConversionsU64ToUsizeUnwrap,
        CuLibraryInstruction::ConversionsU64ToUsizeMapErr,
        CuLibraryInstruction::ConversionsU32ToUsizeUnwrap,
        CuLibraryInstruction::ConversionsU32ToUsizeMapErr,
        CuLibraryInstruction::ConversionsU16ToUsizeUnwrap,
        CuLibraryInstruction::ConversionsU16ToUsizeMapErr,
        CuLibraryInstruction::ConversionsU8ToUsizeUnwrap,
        CuLibraryInstruction::ConversionsU8ToUsizeMapErr,
        // Cast conversions
        CuLibraryInstruction::ConversionsU8AsU16,
        CuLibraryInstruction::ConversionsU8AsU32,
        CuLibraryInstruction::ConversionsU8AsU64,
        CuLibraryInstruction::ConversionsU8AsUsize,
        CuLibraryInstruction::ConversionsU16AsU8,
        CuLibraryInstruction::ConversionsU16AsU32,
        CuLibraryInstruction::ConversionsU16AsU64,
        CuLibraryInstruction::ConversionsU16AsUsize,
        CuLibraryInstruction::ConversionsU32AsU8,
        CuLibraryInstruction::ConversionsU32AsU16,
        CuLibraryInstruction::ConversionsU32AsU64,
        CuLibraryInstruction::ConversionsU32AsUsize,
        CuLibraryInstruction::ConversionsU64AsU8,
        CuLibraryInstruction::ConversionsU64AsU16,
        CuLibraryInstruction::ConversionsU64AsU32,
        CuLibraryInstruction::ConversionsU64AsUsize,
        // Option handling
        CuLibraryInstruction::OptionCheckedAddU8Unwrap,
        CuLibraryInstruction::OptionCheckedAddU8OkOr,
        CuLibraryInstruction::OptionCheckedAddU8OkOrElse,
        CuLibraryInstruction::OptionCheckedAddU8UnwrapOrDefault,
        CuLibraryInstruction::OptionCheckedAddU8UnwrapOr,
        CuLibraryInstruction::OptionSliceGetArrayUnwrap,
        CuLibraryInstruction::OptionSliceGetArrayOkOr,
        CuLibraryInstruction::OptionSliceGetArrayOkOrElse,
        CuLibraryInstruction::OptionSliceGetArrayUnwrapOrDefault,
        CuLibraryInstruction::OptionSliceGetArrayUnwrapOr,
        CuLibraryInstruction::OptionPubkeyRefMapDeref,
        CuLibraryInstruction::OptionPubkeyAsRefMapConvert,
        CuLibraryInstruction::OptionIfLetSomeU8,
        CuLibraryInstruction::OptionIfLetSomeArray,
        CuLibraryInstruction::OptionIfLetSomePubkey,
        CuLibraryInstruction::OptionIfLetSomeArrayRef,
        CuLibraryInstruction::ArrayU8_32Index,
        CuLibraryInstruction::ArrayU8_32Get,
        CuLibraryInstruction::ArrayU8_32GetOkOr,
        CuLibraryInstruction::ArrayU8_32IfLetGet,
        CuLibraryInstruction::ArrayU64_10Index,
        CuLibraryInstruction::ArrayU64_10Get,
        CuLibraryInstruction::ArrayU64_10GetOkOr,
        CuLibraryInstruction::ArrayU64_10IfLetGet,
        CuLibraryInstruction::VecU8_32Index,
        CuLibraryInstruction::VecU8_32Get,
        CuLibraryInstruction::VecU8_32GetOkOr,
        CuLibraryInstruction::VecU8_32IfLetGet,
        CuLibraryInstruction::VecU64_10Index,
        CuLibraryInstruction::VecU64_10Get,
        CuLibraryInstruction::VecU64_10GetOkOr,
        CuLibraryInstruction::VecU64_10IfLetGet,
        // Serialization benchmarks
        CuLibraryInstruction::SerializationBytemuckTryPodReadUnaligned,
        CuLibraryInstruction::SerializationBytemuckPodReadUnaligned,
        CuLibraryInstruction::SerializationBytemuckTryFromBytes,
        CuLibraryInstruction::SerializationCompressedAccountInfoBorshDeserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoZeroCopyDeserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoWincodeDeserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoBincodeDeserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoBorsh1Deserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoRkyvZeroCopyDeserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoWincodeShortVecDeserialize,
        // Seed reference collection benchmarks
        CuLibraryInstruction::CollectVec1Seed,
        CuLibraryInstruction::CollectVec3Seeds,
        CuLibraryInstruction::CollectVec16Seeds,
        CuLibraryInstruction::CollectArrayVec1Seed,
        CuLibraryInstruction::CollectArrayVec3Seeds,
        CuLibraryInstruction::CollectArrayVec16Seeds,
        CuLibraryInstruction::CollectArrayVec3Seeds16Capacity,
        CuLibraryInstruction::Array3Seeds,
        CuLibraryInstruction::Array3SeedsMaybeUninit,
        CuLibraryInstruction::Array3SeedsMaybeUninitNoTransmute,
        CuLibraryInstruction::Array3SeedsPtr,
    ];

    for instruction_type in instructions.into_iter() {
        // Skip instructions that we don't want to test
        if matches!(
            instruction_type,
            CuLibraryInstruction::AccountInfoClose
                | CuLibraryInstruction::AccountInfoCloseUnchecked
        ) {
            continue;
        }

        let (instruction, accounts) = if matches!(
            instruction_type,
            CuLibraryInstruction::CpiAccountMetaArray10
                | CuLibraryInstruction::CpiAccountInfoArray10Ref
                | CuLibraryInstruction::CpiAccountInfoArray10Clone
                | CuLibraryInstruction::CpiAccountInfoArray10Move
                | CuLibraryInstruction::CpiArrayvecPushAccountMeta10
                | CuLibraryInstruction::CpiArrayvecPushAccountInfo10Ref
                | CuLibraryInstruction::CpiArrayvecPushAccountInfo10Clone
                | CuLibraryInstruction::CpiArrayvecPushAccountInfo10Move
                | CuLibraryInstruction::CpiAccountMetaArray10Loop
                | CuLibraryInstruction::CpiAccountInfoArray10RefLoop
                | CuLibraryInstruction::CpiAccountInfoArray10CloneLoop
                | CuLibraryInstruction::CpiAccountInfoArray10MoveLoop
        ) {
            let instr = create_instruction_with_10_accounts(
                program_id,
                instruction_type,
                payer,
                &cpi_pubkeys,
            );
            let mut accs: Vec<(Pubkey, Account)> = cpi_pubkeys
                .iter()
                .map(|pk| {
                    (
                        *pk,
                        Account {
                            lamports: 100_000_000,
                            data: vec![],
                            owner: program_id,
                            executable: false,
                            rent_epoch: 0,
                        },
                    )
                })
                .collect();
            accs.push((payer, payer_account.clone()));
            (instr, accs)
        } else if matches!(
            instruction_type,
            CuLibraryInstruction::AccountInfoKey
                | CuLibraryInstruction::AccountInfoOwner
                | CuLibraryInstruction::AccountInfoIsSigner
                | CuLibraryInstruction::AccountInfoIsWritable
                | CuLibraryInstruction::AccountInfoExecutable
                | CuLibraryInstruction::AccountInfoDataLen
                | CuLibraryInstruction::AccountInfoLamports
                | CuLibraryInstruction::AccountInfoDataIsEmpty
                | CuLibraryInstruction::AccountInfoIsOwnedBy
                | CuLibraryInstruction::AccountInfoAssign
                | CuLibraryInstruction::AccountInfoIsBorrowed
                | CuLibraryInstruction::AccountInfoBorrowLamportsUnchecked
                | CuLibraryInstruction::AccountInfoBorrowMutLamportsUnchecked
                | CuLibraryInstruction::AccountInfoBorrowDataUnchecked
                | CuLibraryInstruction::AccountInfoBorrowMutDataUnchecked
                | CuLibraryInstruction::AccountInfoTryBorrowLamports
                | CuLibraryInstruction::AccountInfoTryBorrowMutLamports
                | CuLibraryInstruction::AccountInfoCanBorrowLamports
                | CuLibraryInstruction::AccountInfoCanBorrowMutLamports
                | CuLibraryInstruction::AccountInfoTryBorrowData
                | CuLibraryInstruction::AccountInfoTryBorrowMutData
                | CuLibraryInstruction::AccountInfoCanBorrowData
                | CuLibraryInstruction::AccountInfoCanBorrowMutData
                | CuLibraryInstruction::AccountInfoRealloc
        ) {
            let instr = create_instruction_with_account(
                program_id,
                instruction_type,
                payer,
                test_account_pubkey,
            );
            let accs = vec![
                (
                    test_account_pubkey,
                    Account {
                        lamports: 1_000_000_000,
                        data: vec![1u8; 1024],
                        owner: program_id,
                        executable: false,
                        rent_epoch: 0,
                    },
                ),
                (payer, payer_account.clone()),
            ];
            (instr, accs)
        } else {
            let instr = create_instruction(program_id, instruction_type, payer);
            let accs = vec![(payer, payer_account.clone())];
            (instr, accs)
        };

        let result = mollusk.process_instruction(&instruction, &accounts);
        println!(
            "instruction {:?}, CU consumed: {}",
            instruction.data, result.compute_units_consumed
        );

        if let Some((func_name, cu_consumed, file_location)) =
            take_profiling_results().into_iter().next()
        {
            let (category, filename) = extract_category_and_file(&file_location);
            results_by_category
                .entry(category)
                .or_default()
                .entry(filename)
                .or_default()
                .push(BenchmarkEntry {
                    func_name,
                    cu_value: cu_consumed.to_string(),
                    file_location,
                });
        }
    }

    // Write results to README.md grouped by category
    let config = ReadmeConfig {
        title: "CU Library Benchmarks".to_string(),
        description: "Benchmark results for Solana runtime operations.".to_string(),
        github_base_url: "https://github.com/Lightprotocol/cu-library/blob/main/".to_string(),
        output_path: "README.md".to_string(),
        display_name_overrides: HashMap::new(),
    };
    write_categorized_readme(&config, results_by_category);

    println!("Benchmark results written to README.md");
}

pub fn create_instruction(
    program_id: Pubkey,
    instruction_type: CuLibraryInstruction,
    payer: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(payer, true)],
        data: instruction_type.into(),
    }
}

pub fn create_instruction_with_account(
    program_id: Pubkey,
    instruction_type: CuLibraryInstruction,
    payer: Pubkey,
    test_account: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(test_account, false),
            AccountMeta::new(payer, true),
        ],
        data: instruction_type.into(),
    }
}

pub fn create_instruction_with_10_accounts(
    program_id: Pubkey,
    instruction_type: CuLibraryInstruction,
    payer: Pubkey,
    cpi_accounts: &[Pubkey],
) -> Instruction {
    let mut accounts = vec![];
    for account in cpi_accounts.iter().take(10) {
        accounts.push(AccountMeta::new_readonly(*account, false));
    }
    accounts.push(AccountMeta::new(payer, true));

    Instruction {
        program_id,
        accounts,
        data: instruction_type.into(),
    }
}
