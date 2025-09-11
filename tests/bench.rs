use cu_library::CuLibraryInstruction;
use litesvm::LiteSVM;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::fs::{self, OpenOptions};
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

    // Create log file
    fs::create_dir_all("target").ok();
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("target/bench.log")
        .expect("Failed to create log file");

    // Write table header
    writeln!(log_file, "Function                | CU Consumed | Heap Bytes Used").unwrap();
    writeln!(log_file, "------------------------|-------------|----------------").unwrap();

    let instructions = vec![CuLibraryInstruction::Msg10];
    for instruction_type in instructions.into_iter() {
        let instruction = create_instruction(program_id, instruction_type, payer.pubkey());
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

        // Parse and write benchmark results to log
        write_benchmark_log(&mut log_file, &meta.logs);
    }

    println!("Benchmark results written to target/bench.log");
}

fn write_benchmark_log(log_file: &mut std::fs::File, logs: &[String]) {
    // Parse the logs to extract profiler output
    for log in logs {
        // Check if this log contains profiler output
        if log.starts_with("Program log:") && log.contains("#") && log.contains("CU") && log.contains("consumed") {
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
                            let func_name = parts[0];
                            
                            // Look for the CU consumption line (2 lines down)
                            let mut cu_value = "N/A";
                            let mut heap_value = "0";
                            
                            if i + 2 < lines.len() {
                                if let Some(cu_line) = lines.get(i + 2) {
                                    if cu_line.contains("CU") && cu_line.contains("consumed") {
                                        // Extract CU consumed value
                                        if let Some(consumed_pos) = cu_line.find("consumed") {
                                            let after_consumed = &cu_line[consumed_pos + 8..].trim();
                                            // Find the number after "consumed"
                                            let parts: Vec<&str> = after_consumed.split_whitespace().collect();
                                            if !parts.is_empty() {
                                                cu_value = parts[0];
                                            }
                                        }
                                        
                                        // Look for heap usage in the same line
                                        if let Some(heap_pos) = cu_line.find("heap") {
                                            let after_heap = &cu_line[heap_pos + 4..].trim();
                                            // Extract heap value
                                            let parts: Vec<&str> = after_heap.split_whitespace().collect();
                                            if !parts.is_empty() {
                                                heap_value = parts[0];
                                            }
                                        }
                                    }
                                }
                            }
                            
                            writeln!(
                                log_file,
                                "{:<24}| {:<11} | {}",
                                func_name, cu_value, heap_value
                            )
                            .unwrap();
                        }
                    }
                }
            }
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
