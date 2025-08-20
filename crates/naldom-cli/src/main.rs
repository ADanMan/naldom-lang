// crates/naldom-cli/src/main.rs

use clap::Parser;
use naldom_core::codegen_llvm::generate_llvm_ir;
use naldom_core::llm_inference::run_inference;
use naldom_core::lowering::LoweringContext;
use naldom_core::lowering_hl_to_ll::lower_hl_to_ll;
use naldom_core::parser::parse_to_intent_graph;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

/// The Naldom Compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input .md or .nldm file
    file_path: PathBuf,

    /// Output file path
    #[arg(short, long, default_value = "a.out")]
    output: PathBuf,

    /// Enable trace logging to view intermediate representations
    #[arg(long)]
    trace: bool,

    /// Compile and run the program immediately
    #[arg(long)]
    run: bool,

    /// Emit a specific intermediate representation
    #[arg(long, value_name = "FORMAT")]
    emit: Option<String>,
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
        println!(
            "========== 2. High-Level IR (IR-HL) ==========\n{:#?}\n============================================\n",
            hl_program
        );
    }

    // 5. Lower High-Level IR to Low-Level IR (IR-LL)
    let ll_program = lower_hl_to_ll(&hl_program);
    if args.trace {
        println!(
            "========== 3. Low-Level IR (IR-LL) ==========\n{:#?}\n===========================================\n",
            ll_program
        );
    }

    // 6. Generate LLVM IR from Low-Level IR
    let llvm_ir = match generate_llvm_ir(&ll_program) {
        Ok(ir) => ir,
        Err(e) => {
            eprintln!("Error during LLVM IR generation: {}", e);
            exit(1);
        }
    };

    // 7. Handle output based on flags
    if let Some(emit_format) = args.emit {
        if emit_format == "llvm-ir" {
            println!("{}", llvm_ir);
            return; // Exit after emitting
        } else {
            eprintln!("Unsupported emit format: {}", emit_format);
            exit(1);
        }
    }

    // 8. Compile to native executable
    if let Err(e) = compile_native(&llvm_ir, &args.output) {
        eprintln!("Failed to compile native executable: {}", e);
        exit(1);
    }

    println!("Successfully compiled to '{}'", args.output.display());

    // 9. Handle --run flag
    if args.run {
        println!("\nRunning '{}'...\n", args.output.display());
        let output = Command::new(&args.output)
            .output()
            .expect("Failed to execute compiled program");

        // Print stdout and stderr of the child process
        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

/// Orchestrates the native build process: LLVM IR -> .o -> executable
fn compile_native(llvm_ir: &str, output_path: &Path) -> Result<(), String> {
    let (llc_path, clang_path) = match env::var("LLVM_PREFIX") {
        Ok(prefix) => {
            println!("Found LLVM_PREFIX: {}", prefix);
            let llvm_path = PathBuf::from(prefix);
            (llvm_path.join("bin/llc"), llvm_path.join("bin/clang"))
        }
        Err(_) => {
            println!("LLVM_PREFIX not set. Assuming 'llc' and 'clang' are in PATH.");
            (PathBuf::from("llc"), PathBuf::from("clang"))
        }
    };

    let temp_dir = std::env::temp_dir();
    let stem = output_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("naldom_out"));

    // 1. Write LLVM IR to a temporary file
    let ll_path = temp_dir.join(format!("{}.ll", stem.to_str().unwrap()));
    fs::write(&ll_path, llvm_ir)
        .map_err(|e| format!("Failed to write LLVM IR to temp file: {}", e))?;

    // 2. Compile .ll to .o using `llc`
    let obj_path = temp_dir.join(format!("{}.o", stem.to_str().unwrap()));
    let llc_output = Command::new(&llc_path)
        .arg("-filetype=obj")
        .arg(&ll_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute llc-17. Is LLVM 17 installed and in your PATH? Error: {}",
                e
            )
        })?;

    if !llc_output.status.success() {
        return Err(format!(
            "llc failed:\n{}",
            String::from_utf8_lossy(&llc_output.stderr)
        ));
    }

    // 3. Link .o with C runtime using `clang`
    let runtime_path = "runtime/native/naldom_runtime.c";
    let clang_output = Command::new(&clang_path)
        .arg(&obj_path)
        .arg(runtime_path)
        .arg("-o")
        .arg(output_path)
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute clang. Is it installed and in your PATH? Error: {}",
                e
            )
        })?;

    if !clang_output.status.success() {
        return Err(format!(
            "clang failed:\n{}",
            String::from_utf8_lossy(&clang_output.stderr)
        ));
    }

    // 4. Clean up temporary files
    let _ = fs::remove_file(&ll_path);
    let _ = fs::remove_file(&obj_path);

    Ok(())
}
