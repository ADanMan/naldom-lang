// crates/naldom-cli/src/main.rs

use clap::Parser;
use naldom_core::llm_inference::run_inference;
use std::fs;
use std::path::PathBuf;

/// A next-generation programming language based on Natural Language.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the .md or .nldm file to process
    #[arg(required = true)]
    file_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse command-line arguments
    let args = Args::parse();

    println!("Processing file: {:?}", args.file_path);

    // 2. Read the file content into a string
    let file_content = fs::read_to_string(&args.file_path)?;

    // 3. Send the content to the LLM for processing
    println!("\n--- Sending content to LLM ---");

    // 4. Call the inference function from naldom-core and store the result
    let llm_response = run_inference(&file_content)?;

    // 5. Print the fully collected response at the end
    println!("\n\n--- Full LLM Response ---");
    println!("{}", llm_response);
    println!("--- End of Response ---");

    println!("\n--- CLI finished ---");

    Ok(())
}