// crates/naldom-cli/src/main.rs

use clap::Parser;
use naldom_core::llm_inference::run_inference;
use naldom_core::lowering::LoweringContext;
use naldom_core::parser::parse_to_intent_graph;
use std::fs;
use std::path::PathBuf;

/// A next-generation programming language based on Natural Language.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the .md or .nldm file to process
    #[arg(required = true)]
    file_path: PathBuf,

    /// Enable trace mode to see compilation stages
    #[arg(long)]
    trace: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();

    println!("Processing file: {:?}", args.file_path);

    // Read the file content into a string
    let file_content = fs::read_to_string(&args.file_path)?;

    // === Stage 1: LLM Inference ===
    println!("\n--- Stage 1: Sending content to LLM ---");
    let llm_response = run_inference(&file_content)?;
    
    if args.trace {
        println!("\n[TRACE] Full LLM Response:");
        println!("{}", llm_response);
        println!("[TRACE] --- End of LLM Response ---");
    }

    // === Stage 2: Parsing to IntentGraph ===
    println!("\n--- Stage 2: Parsing response to IntentGraph ---");
    let intent_graph = parse_to_intent_graph(&llm_response)?;
    
    if args.trace {
        println!("\n[TRACE] IntentGraph Output:");
        dbg!(&intent_graph);
    }

    // === Stage 3: Lowering to High-Level IR ===
    println!("\n--- Stage 3: Lowering IntentGraph to IR-HL ---");
    let mut lowering_context = LoweringContext::new();
    let hl_program = lowering_context.lower(&intent_graph);

    if args.trace {
        println!("\n[TRACE] IR-HL Output:");
        dbg!(&hl_program);
    }

    println!("\n--- Compilation finished ---");

    Ok(())
}
