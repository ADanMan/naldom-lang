// crates/naldom-cli/src/main.rs

use clap::Parser;
use naldom_core::llm_inference::run_inference;
use naldom_core::lowering::LoweringContext;
use naldom_core::lowering_hl_to_ll::lower_hl_to_ll;
use naldom_core::parser::parse_to_intent_graph;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

/// The Naldom Compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input .md or .nldm file
    file_path: PathBuf,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Enable trace logging to view intermediate representations
    #[arg(long)]
    trace: bool,

    /// Compile and run the program immediately
    #[arg(long)]
    run: bool,
}

fn main() {
    let args = Args::parse();

    // 1. Read the source file
    let source_code = match fs::read_to_string(&args.file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args.file_path.display(), e);
            exit(1);
        }
    };

    // 2. Run LLM Inference to get JSON
    let llm_response = match run_inference(&source_code) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error during LLM inference: {}", e);
            exit(1);
        }
    };

    // 3. Parse JSON into IntentGraph
    let intent_graph = match parse_to_intent_graph(&llm_response) {
        Ok(graph) => graph,
        Err(e) => {
            eprintln!("Error parsing LLM response into IntentGraph: {}", e);
            eprintln!("--- LLM Response ---");
            eprintln!("{}", llm_response);
            eprintln!("--------------------");
            exit(1);
        }
    };

    if args.trace {
        println!("\n========== 1. IntentGraph ==========");
        println!("{:#?}", intent_graph);
        println!("==================================\n");
    }

    // 4. Lower IntentGraph to High-Level IR (IR-HL)
    let mut lowering_context = LoweringContext::new();
    let hl_program = lowering_context.lower(&intent_graph);

    if args.trace {
        println!("========== 2. High-Level IR (IR-HL) ==========");
        println!("{:#?}", hl_program);
        println!("============================================\n");
    }

    // 5. Lower High-Level IR to Low-Level IR (IR-LL)
    let ll_program = lower_hl_to_ll(&hl_program);

    if args.trace {
        println!("========== 3. Low-Level IR (IR-LL) ==========");
        println!("{:#?}", ll_program);
        println!("===========================================\n");
    }

    // The rest of the pipeline (codegen, etc.) will be modified in future steps.
    // For now, we stop here.
    println!("Compiler finished successfully up to IR-LL generation.");
}
