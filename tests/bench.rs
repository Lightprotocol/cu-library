use cu_library::CuLibraryInstruction;
use litesvm::LiteSVM;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;

#[test]
fn bench_cu_operations() {
    // Get the path from SBF_OUT_DIR environment variable (set by cargo test-sbf)
    let program_path = std::env::var("SBF_OUT_DIR")
        .map(|dir| format!("{}/cu_library.so", dir))
        .unwrap_or_else(|_| "target/deploy/cu_library.so".to_string());

    // Setup
    let mut svm = LiteSVM::new();
    let program_id = Pubkey::new_unique();
    svm.add_program_from_file(program_id, program_path)
        .expect("Failed to load program");

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    // Create a test account with 1KB of data for AccountInfo benchmarks
    let test_account = Keypair::new();

    // Collect benchmark results by category
    let mut results_by_category: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();

    let instructions = vec![
        CuLibraryInstruction::Baseline,
        CuLibraryInstruction::Msg10,
        CuLibraryInstruction::SolanaMsg10,
        CuLibraryInstruction::SolanaMsgProgramId,
        CuLibraryInstruction::SolanaPubkeyNewFromArray,
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
        // Skip close operations as they would affect subsequent tests
        CuLibraryInstruction::AccountInfoClose,
        CuLibraryInstruction::AccountInfoCloseUnchecked,
    ];

    for instruction_type in instructions.into_iter() {
        let instruction = if matches!(
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
                | CuLibraryInstruction::AccountInfoClose
                | CuLibraryInstruction::AccountInfoCloseUnchecked
        ) {
            let test_account_data = vec![1u8; 1024]; // 1KB of 1u8
            svm.set_account(
                test_account.pubkey(),
                Account {
                    lamports: 1_000_000_000,
                    data: test_account_data,
                    owner: program_id,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();

            create_instruction_with_account(
                program_id,
                instruction_type,
                payer.pubkey(),
                test_account.pubkey(),
            )
        } else {
            create_instruction(program_id, instruction_type, payer.pubkey())
        };
        println!("instruction {:?}", instruction);
        let blockhash = svm.latest_blockhash();

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            blockhash,
        );

        let meta = svm.send_transaction(tx).unwrap();
        let logs = meta.pretty_logs();
        println!("{}", logs);

        // Parse benchmark results
        if let Some((func_name, cu_value)) = parse_benchmark_log(&meta.logs) {
            // Determine category from function name prefix
            let category = func_name.split('_').next().unwrap_or("other").to_string();
            results_by_category
                .entry(category)
                .or_insert_with(Vec::new)
                .push((func_name, cu_value));
        }
    }

    // Write results to README.md grouped by category
    write_categorized_readme(results_by_category);

    println!("Benchmark results written to README.md");
}

fn parse_benchmark_log(logs: &[String]) -> Option<(String, String)> {
    // Parse the logs to extract profiler output
    for log in logs {
        // Check if this log contains profiler output
        if log.starts_with("Program log:")
            && log.contains("#")
            && log.contains("CU")
            && log.contains("consumed")
        {
            // Remove "Program log: " prefix and split by newlines
            let content = log.strip_prefix("Program log: ").unwrap_or(log);
            let lines: Vec<&str> = content.lines().collect();

            for (i, line) in lines.iter().enumerate() {
                // Look for lines starting with "#" which indicate profiler function names
                if line.contains("#") && line.contains("    ") {
                    // Extract function name - it's after the # and number
                    let trimmed = line.trim();
                    if let Some(start) = trimmed.find("    ") {
                        let func_part = &trimmed[start..].trim();
                        let parts: Vec<&str> = func_part.split_whitespace().collect();
                        if !parts.is_empty() {
                            let func_name = parts[0].to_string();

                            // Look for the CU consumption line (2 lines down)
                            let mut cu_value = "N/A".to_string();

                            if i + 2 < lines.len() {
                                if let Some(cu_line) = lines.get(i + 2) {
                                    if cu_line.contains("CU") && cu_line.contains("consumed") {
                                        // Extract CU consumed value
                                        if let Some(consumed_pos) = cu_line.find("consumed") {
                                            let after_consumed =
                                                &cu_line[consumed_pos + 8..].trim();
                                            // Find the number after "consumed"
                                            let parts: Vec<&str> =
                                                after_consumed.split_whitespace().collect();
                                            if !parts.is_empty() {
                                                cu_value = parts[0].to_string();
                                            }
                                        }
                                    }
                                }
                            }

                            return Some((func_name, cu_value));
                        }
                    }
                }
            }
        }
    }
    None
}

fn write_categorized_readme(mut results_by_category: BTreeMap<String, Vec<(String, String)>>) {
    let mut readme = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("README.md")
        .expect("Failed to create README.md");

    // Write README header
    writeln!(readme, "# CU Library Benchmarks\n").unwrap();
    writeln!(readme, "Benchmark results for Solana runtime operations:\n").unwrap();
    writeln!(
        readme,
        "**Note:** The `#[profile]` macro adds ~5-6 CU overhead to each measurement.\n"
    )
    .unwrap();

    // Write Baseline category first if it exists
    if let Some(baseline_results) = results_by_category.remove("baseline") {
        writeln!(readme, "## Baseline\n").unwrap();

        // Write table header
        writeln!(
            readme,
            "| Function                                    | CU Consumed |"
        )
        .unwrap();
        writeln!(
            readme,
            "|---------------------------------------------|-------------|"
        )
        .unwrap();

        // Write results
        for (func_name, cu_value) in baseline_results {
            writeln!(readme, "| {:<43} | {:<11} |", func_name, cu_value).unwrap();
        }

        writeln!(readme).unwrap(); // Empty line after baseline
    }

    // Write remaining categories
    for (category, results) in results_by_category {
        // Format category name (capitalize first letter)
        let category_name = format!(
            "{}{}",
            category.chars().next().unwrap().to_uppercase(),
            &category[1..]
        );

        writeln!(readme, "## {}\n", category_name).unwrap();

        // Write table header
        writeln!(
            readme,
            "| Function                                    | CU Consumed |"
        )
        .unwrap();
        writeln!(
            readme,
            "|---------------------------------------------|-------------|"
        )
        .unwrap();

        // Write results
        for (func_name, cu_value) in results {
            writeln!(readme, "| {:<43} | {:<11} |", func_name, cu_value).unwrap();
        }

        writeln!(readme).unwrap(); // Empty line between categories
    }
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
