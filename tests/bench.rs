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

    // Create test accounts for AccountInfo and CPI benchmarks
    let test_account = Keypair::new();

    // Create 10 accounts for CPI benchmarks
    let cpi_accounts: Vec<Keypair> = (0..10).map(|_| Keypair::new()).collect();
    for account in &cpi_accounts {
        svm.set_account(
            account.pubkey(),
            Account {
                lamports: 100_000_000,
                data: vec![],
                owner: program_id,
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();
    }

    // Collect benchmark results by category and file
    // Structure: folder -> file -> [(func_name, cu_value, file_location)]
    let mut results_by_category: BTreeMap<String, BTreeMap<String, Vec<(String, String, String)>>> = BTreeMap::new();

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
        CuLibraryInstruction::SerializationCompressedAccountInfoBorshDeserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoZeroCopyDeserialize,
        CuLibraryInstruction::SerializationCompressedAccountInfoWincodeDeserialize,
    ];

    for instruction_type in instructions.into_iter() {
        // Skip instructions that we don't want to test
        if matches!(
            instruction_type,
            CuLibraryInstruction::AccountInfoClose | CuLibraryInstruction::AccountInfoCloseUnchecked
        ) {
            continue;
        }
        
        let instruction = if matches!(
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
            let cpi_pubkeys: Vec<Pubkey> = cpi_accounts.iter().map(|k| k.pubkey()).collect();
            create_instruction_with_10_accounts(
                program_id,
                instruction_type,
                payer.pubkey(),
                &cpi_pubkeys,
            )
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
        if let Some((func_name, cu_value, file_location)) = parse_benchmark_log(&meta.logs) {
            // Determine category (folder) and file from file path
            let (category, filename) = extract_category_and_file_from_path(&file_location);
            results_by_category
                .entry(category)
                .or_insert_with(BTreeMap::new)
                .entry(filename)
                .or_insert_with(Vec::new)
                .push((func_name, cu_value, file_location));
        }
    }

    // Write results to README.md grouped by category
    write_categorized_readme(results_by_category);

    println!("Benchmark results written to README.md");
}

/// Extract category hierarchy from file path
/// Returns (folder_name, file_stem) for grouping
fn extract_category_and_file_from_path(file_location: &str) -> (String, String) {
    // Handle special case: baseline functions in lib.rs
    if file_location == "src/lib.rs" || file_location.starts_with("src/lib.rs:") {
        return ("baseline".to_string(), "lib".to_string());
    }

    // Extract folder name and file from path
    // Format: "src/folder_name/file.rs:line_number"
    if file_location.starts_with("src/") {
        let without_src = &file_location[4..]; // Remove "src/"
        let path_parts: Vec<&str> = without_src.split('/').collect();

        if path_parts.len() >= 2 {
            // Get the folder name (first part)
            let folder_name = path_parts[0];

            // Get the file name (second part)
            let file_part = path_parts[1];
            // Remove .rs extension and line number
            let file_stem = file_part
                .split(':')
                .next()
                .unwrap_or(file_part)
                .trim_end_matches(".rs");

            return (folder_name.to_string(), file_stem.to_string());
        } else if !path_parts.is_empty() {
            // Only folder, no file (shouldn't happen but handle it)
            let folder_name = path_parts[0];
            let clean_folder = folder_name.split('.').next().unwrap_or(folder_name);
            let clean_folder = clean_folder.split(':').next().unwrap_or(clean_folder);
            return (clean_folder.to_string(), "unknown".to_string());
        }
    }

    // Fallback
    ("other".to_string(), "unknown".to_string())
}

/// Format folder name for display in README sections
fn format_display_name(folder_name: &str) -> String {
    match folder_name {
        "account_info" => "Account Info".to_string(),
        "pinocchio_ops" => "Pinocchio Ops".to_string(),
        "solana_ops" => "Solana Ops".to_string(),
        "checked_math" => "Checked Math".to_string(),
        "saturating_math" => "Saturating Math".to_string(),
        "std_math" => "Std Math".to_string(),
        "partial_eq" => "Partial Eq".to_string(),
        "vec_access" => "Vec Access".to_string(),
        "baseline" => "Baseline".to_string(),
        _ => {
            // Default: capitalize first letter
            let mut chars = folder_name.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => folder_name.to_string(),
            }
        }
    }
}

/// Format file name for display in README subsections
fn format_file_display_name(file_stem: &str) -> String {
    // Convert snake_case to Title Case
    file_stem
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Add indentation based on nesting level
/// Level 0 (##): no indentation
/// Level 1 (###): 2 spaces
/// Uses normal spaces for indentation
fn add_indentation(text: &str, level: usize) -> String {
    let indent = "  ".repeat(level);
    format!("{}{}", indent, text)
}

fn parse_benchmark_log(logs: &[String]) -> Option<(String, String, String)> {
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
                            
                            // Look for the file location line (next line)
                            let mut file_location = String::new();
                            if i + 1 < lines.len() {
                                if let Some(location_line) = lines.get(i + 1) {
                                    // The location line contains "src/..." 
                                    let location_trimmed = location_line.trim();
                                    if location_trimmed.starts_with("src/") {
                                        file_location = location_trimmed.to_string();
                                    }
                                }
                            }

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

                            return Some((func_name, cu_value, file_location));
                        }
                    }
                }
            }
        }
    }
    None
}

fn write_categorized_readme(mut results_by_category: BTreeMap<String, BTreeMap<String, Vec<(String, String, String)>>>) {
    let mut readme = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("README.md")
        .expect("Failed to create README.md");

    // Write README header
    writeln!(readme, "# CU Library Benchmarks\n").unwrap();
    writeln!(readme, "Benchmark results for Solana runtime operations.\n").unwrap();

    // Generate table of contents with numbering
    writeln!(readme, "## Table of Contents\n").unwrap();

    // Add Baseline to TOC first if it exists (number 1)
    let mut section_number = 1;
    let mut baseline_number = 0;
    if results_by_category.contains_key("baseline") {
        writeln!(readme, "**[{}. Baseline](#{}-baseline)**\n", section_number, section_number).unwrap();
        baseline_number = section_number;
        section_number += 1;
    }

    // Add all other categories to TOC with numbering
    let mut category_numbers = BTreeMap::new();
    for category in results_by_category.keys() {
        if category != "baseline" {
            let display_name = format_display_name(category);
            let anchor = format!("{}-{}", section_number, category.replace('_', "-"));
            writeln!(readme, "**[{}. {}](#{})**\n", section_number, display_name, anchor).unwrap();
            category_numbers.insert(category.clone(), section_number);
            section_number += 1;
        }
    }
    
    writeln!(readme).unwrap(); // Empty line after TOC
    
    // Write definitions
    writeln!(readme, "## Definitions\n").unwrap();
    writeln!(
        readme,
        "- **CU Consumed**: Total compute units consumed by the profiled function"
    )
    .unwrap();
    writeln!(
        readme,
        "- **CU Adjusted**: Actual operation cost with baseline profiling overhead subtracted (CU Consumed - Baseline CU)"
    )
    .unwrap();
    writeln!(
        readme,
        "- **Baseline CU**: CU consumed by an empty profiled function (`#[profile]` macro overhead)\n"
    )
    .unwrap();

    // Get baseline CU value for adjustment
    let mut baseline_cu: u64 = 0;
    if let Some(baseline_files) = results_by_category.get("baseline") {
        // Get the first function from the first file
        if let Some(first_file_results) = baseline_files.values().next() {
            if let Some((_, cu_str, _)) = first_file_results.first() {
                baseline_cu = cu_str.parse::<u64>().unwrap_or(0);
            }
        }
    }

    // Write Baseline category first if it exists
    if let Some(baseline_files) = results_by_category.remove("baseline") {
        writeln!(readme, "## {}. Baseline\n", baseline_number).unwrap();

        // Iterate through each file in baseline
        let mut file_number = 1;
        for (file_stem, results) in baseline_files {
            let file_display_name = format_file_display_name(&file_stem);
            let indented_header = add_indentation(&format!("### {}.{} {}", baseline_number, file_number, file_display_name), 1);
            writeln!(readme, "{}\n", indented_header).unwrap();

            // Write table header with indentation (same width as other tables)
            let table_header = add_indentation("| Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |", 1);
            let table_separator = add_indentation("|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|", 1);
            writeln!(readme, "{}", table_header).unwrap();
            writeln!(readme, "{}", table_separator).unwrap();

            // Write results
            for (func_name, cu_value, file_location) in results {
                // Create GitHub link without extra indentation
                let github_link = if !file_location.is_empty() {
                    // Extract file path and line number (format: "src/path/file.rs:line")
                    let parts: Vec<&str> = file_location.split(':').collect();
                    if parts.len() >= 2 {
                        let file_path = parts[0];
                        let line_num = parts[1].trim().parse::<usize>().unwrap_or(0) + 1;
                        format!("[{}](https://github.com/Lightprotocol/cu-library/blob/master/{}#L{})",
                                func_name, file_path, line_num)
                    } else {
                        func_name.clone()
                    }
                } else {
                    func_name.clone()
                };
                // Baseline has no adjustment, show N/A
                let table_row = add_indentation(&format!("| {:<180} | {:<11} | {:<11} |", github_link, cu_value, "N/A"), 1);
                writeln!(readme, "{}", table_row).unwrap();
            }

            writeln!(readme).unwrap(); // Empty line after file section
            file_number += 1;
        }
    }

    // Write remaining categories with numbering
    for (category, files_map) in results_by_category {
        let display_name = format_display_name(&category);
        let number = category_numbers.get(&category).unwrap_or(&0);

        writeln!(readme, "## {}. {}\n", number, display_name).unwrap();

        // Iterate through each file in the category
        let mut file_number = 1;
        for (file_stem, results) in files_map {
            let file_display_name = format_file_display_name(&file_stem);
            let indented_header = add_indentation(&format!("### {}.{} {}", number, file_number, file_display_name), 1);
            writeln!(readme, "{}\n", indented_header).unwrap();

            // Write table header with indentation
            let table_header = add_indentation("| Function                                                                                                                                                                                                                | CU Consumed | CU Adjusted |", 1);
            let table_separator = add_indentation("|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|-------------|", 1);
            writeln!(readme, "{}", table_header).unwrap();
            writeln!(readme, "{}", table_separator).unwrap();

            // Write results
            for (func_name, cu_value, file_location) in results {
                // Create GitHub link without extra indentation
                let github_link = if !file_location.is_empty() {
                    // Extract file path and line number (format: "src/path/file.rs:line")
                    let parts: Vec<&str> = file_location.split(':').collect();
                    if parts.len() >= 2 {
                        let file_path = parts[0];
                        let line_num = parts[1].trim().parse::<usize>().unwrap_or(0) + 1;
                        format!("[{}](https://github.com/Lightprotocol/cu-library/blob/master/{}#L{})",
                                func_name, file_path, line_num)
                    } else {
                        func_name.clone()
                    }
                } else {
                    func_name.clone()
                };

                // Calculate adjusted CU value
                let cu_consumed = cu_value.parse::<u64>().unwrap_or(0);
                let cu_adjusted = if cu_consumed >= baseline_cu {
                    (cu_consumed - baseline_cu).to_string()
                } else {
                    "0".to_string()
                };

                let table_row = add_indentation(&format!("| {:<180} | {:<11} | {:<11} |", github_link, cu_value, cu_adjusted), 1);
                writeln!(readme, "{}", table_row).unwrap();
            }

            writeln!(readme).unwrap(); // Empty line after file section
            file_number += 1;
        }
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
