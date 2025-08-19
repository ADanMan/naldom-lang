// crates/naldom-cli/src/main.rs

use clap::Parser;
use naldom_core::codegen_python::PythonCodeGenerator;
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

    /// Specify the output file path
    #[arg(short, long)]
    output: Option<PathBuf>,
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
    let mut lowering_context = LoweringContext::default(); // This is correct because it has fields
    let hl_program = lowering_context.lower(&intent_graph);

    if args.trace {
        println!("\n[TRACE] IR-HL Output:");
        dbg!(&hl_program);
    }

    // === Stage 4: Generating Python code ===
    println!("\n--- Stage 4: Generating Python code ---");
    // THIS IS THE FIX: For a unit struct, we just use its name to create an instance.
    let codegen = PythonCodeGenerator;
    let generated_code = codegen.generate(&hl_program);

    if args.trace {
        println!("\n[TRACE] Generated Python Code (logic only):");
        println!("{}", generated_code);
    }

    // === Stage 5: Assembling and writing to output file ===
    if let Some(output_path) = &args.output {
        println!("\n--- Stage 5: Assembling and writing output file ---");

        // Embed the runtime code directly into the binary
        let runtime_code = include_str!("../../../runtime/python/naldom_runtime.py");

        // Combine the runtime and the generated code
        let final_code = format!(
            "# -- Naldom Python Runtime --\n{}\n\n# --- Generated Code ---\n{}",
            runtime_code, generated_code
        );

        // Write to the specified file
        fs::write(output_path, final_code)?;
        println!("Successfully wrote Python script to: {:?}", output_path);
    }

    println!("\n--- Compilation finished ---");
    Ok(())
}
