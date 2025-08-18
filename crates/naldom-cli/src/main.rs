// crates/naldom-cli/src/main.rs

use clap::Parser;
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
    // 1. Parsing command line arguments
    let args = Args::parse();

    println!("Processing file: {:?}", args.file_path);

    // 2. Reading the contents of a file into a string
    //    Use `?` for elegant error handling: if the file is not found or cannot be read, the program will exit with an error.
    let file_content = fs::read_to_string(&args.file_path)
        .expect(&format!("Failed to read file: {:?}", args.file_path));

    // 3. We output the contents of the file to the console for verification
    println!("\n--- File Content ---");
    println!("{}", file_content);
    println!("--- End of File ---");

    Ok(())
}