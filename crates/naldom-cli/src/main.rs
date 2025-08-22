// crates/naldom-cli/src/main.rs

use clap::Parser;
use naldom_core::codegen_llvm::generate_llvm_ir;
use naldom_core::llm_inference::run_inference;
use naldom_core::lowering::LoweringContext;
use naldom_core::lowering_hl_to_ll::lower_hl_to_ll;
use naldom_core::parser::parse_to_intent_graph;
use naldom_core::semantic_analyzer::SemanticAnalyzer;
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

    /// Output file path. Defaults to 'a.out' for native and 'a.out.wasm' for wasm.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// The compilation target ('native' or 'wasm')
    #[arg(long, default_value = "native")]
    target: String,

    /// Optimization level (0-3)
    #[arg(short = 'O', long, default_value = "0")]
    opt_level: u8,

    /// Enable trace logging to view intermediate representations
    #[arg(long)]
    trace: bool,

    /// Compile and run the program immediately (native target only)
    #[arg(long)]
    run: bool,

    /// Emit a specific intermediate representation
    #[arg(long, value_name = "FORMAT")]
    emit: Option<String>,
}

fn main() {
    let args = Args::parse();
    let output_path = args.output.clone().unwrap_or_else(|| {
        if args.target == "wasm" {
            PathBuf::from("a.out.wasm")
        } else {
            PathBuf::from("a.out")
        }
    });

    // 1. Read the source file
    let source_code = fs::read_to_string(&args.file_path).unwrap_or_else(|e| {
        eprintln!("Error reading file '{}': {}", args.file_path.display(), e);
        exit(1);
    });

    // 2. Run LLM Inference to get JSON
    let llm_response = run_inference(&source_code).unwrap_or_else(|e| {
        eprintln!("Error during LLM inference: {}", e);
        exit(1);
    });

    // 3. Parse JSON into IntentGraph
    let intent_graph = parse_to_intent_graph(&llm_response).unwrap_or_else(|e| {
        eprintln!("Error parsing LLM response into IntentGraph: {}", e);
        eprintln!(
            "--- LLM Response ---\n{}\n--------------------",
            llm_response
        );
        exit(1);
    });
    if args.trace {
        println!(
            "\n========== 1. IntentGraph (Parsed) ==========\n{:#?}\n=========================================\n",
            intent_graph
        );
    }

    // 3.5 Semantic Analysis
    let mut analyzer = SemanticAnalyzer::new();
    let validated_intent_graph = analyzer.analyze(&intent_graph).unwrap_or_else(|e| {
        eprintln!("Compilation failed: {}", e);
        exit(1);
    });
    if args.trace {
        println!(
            "========== 1.5. IntentGraph (Validated) ==========\n{:#?}\n==============================================\n",
            validated_intent_graph
        );
    }

    // 4. Lower IntentGraph to High-Level IR (IR-HL)
    let mut lowering_context = LoweringContext::new();
    let hl_program = lowering_context.lower(&validated_intent_graph);
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

    // 6. Determine target triple for LLVM IR generation
    let target_triple_string = if args.target == "wasm" {
        "wasm32-unknown-unknown".to_string()
    } else {
        inkwell::targets::TargetMachine::get_default_triple()
            .as_str()
            .to_str()
            .unwrap_or("unknown-unknown-unknown")
            .to_string()
    };

    // 7. Generate LLVM IR (without optimization at this stage)
    let llvm_ir = generate_llvm_ir(&ll_program, &target_triple_string).unwrap_or_else(|e| {
        eprintln!("Error during LLVM IR generation: {}", e);
        exit(1);
    });

    // 8. Handle --emit flag
    if let Some(emit_format) = &args.emit {
        if emit_format == "llvm-ir" {
            println!("{}", llvm_ir);
            return;
        } else {
            eprintln!("Unsupported emit format: {}", emit_format);
            exit(1);
        }
    }

    // 9. Compile to the specified target (with optimizations applied by external tools)
    let compile_result = if args.target == "wasm" {
        compile_wasm(&llvm_ir, &output_path, args.opt_level)
    } else {
        compile_native(&llvm_ir, &output_path, args.opt_level)
    };

    if let Err(e) = compile_result {
        eprintln!("Failed to compile for target '{}': {}", args.target, e);
        exit(1);
    }

    println!("Successfully compiled to '{}'", output_path.display());

    // 10. Handle --run flag (only for native target)
    if args.run {
        if args.target == "wasm" {
            println!(
                "\nCannot run wasm target directly. Use a Wasm runtime like wasmtime or a browser."
            );
        } else {
            println!("\nRunning '{}'...\n", output_path.display());
            let output = Command::new(&output_path)
                .output()
                .expect("Failed to execute compiled program");

            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }
}

/// Orchestrates the native build process: LLVM IR -> .o -> executable
fn compile_native(llvm_ir: &str, output_path: &Path, opt_level: u8) -> Result<(), String> {
    let (llc_path, clang_path) = match env::var("LLVM_PREFIX") {
        Ok(prefix) => {
            let llvm_path = PathBuf::from(prefix);
            (llvm_path.join("bin/llc"), llvm_path.join("bin/clang"))
        }
        Err(_) => (PathBuf::from("llc"), PathBuf::from("clang")),
    };

    let temp_dir = std::env::temp_dir();
    let stem = output_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("naldom_out"));

    let ll_path = temp_dir.join(format!("{}.ll", stem.to_str().unwrap()));
    fs::write(&ll_path, llvm_ir)
        .map_err(|e| format!("Failed to write LLVM IR to temp file: {}", e))?;

    let obj_path = temp_dir.join(format!("{}.o", stem.to_str().unwrap()));

    let opt_flag = format!("-O{}", opt_level); // Optimization flag for llc

    let llc_output = Command::new(&llc_path)
        .arg(&opt_flag) // Apply optimization level
        .arg("-filetype=obj")
        .arg(&ll_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute '{}'. Is LLVM installed and configured correctly? Error: {}",
                llc_path.display(),
                e
            )
        })?;

    if !llc_output.status.success() {
        return Err(format!(
            "llc failed:\n{}",
            String::from_utf8_lossy(&llc_output.stderr)
        ));
    }

    let runtime_path = "runtime/native/naldom_runtime.c";
    let clang_output = Command::new(&clang_path)
        .arg(&obj_path)
        .arg(runtime_path)
        .arg("-o")
        .arg(output_path)
        .arg(&opt_flag) // Also pass optimization level to clang for linking optimizations
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute '{}'. Is clang installed and configured correctly? Error: {}",
                clang_path.display(),
                e
            )
        })?;

    if !clang_output.status.success() {
        return Err(format!(
            "clang failed:\n{}",
            String::from_utf8_lossy(&clang_output.stderr)
        ));
    }

    let _ = fs::remove_file(&ll_path);
    let _ = fs::remove_file(&obj_path);

    Ok(())
}

/// Orchestrates the WebAssembly build process: LLVM IR -> .o -> .wasm
fn compile_wasm(llvm_ir: &str, output_path: &Path, opt_level: u8) -> Result<(), String> {
    let (llc_path, wasm_ld_path) = match env::var("LLVM_PREFIX") {
        Ok(prefix) => {
            let llvm_path = PathBuf::from(prefix);
            (llvm_path.join("bin/llc"), llvm_path.join("bin/wasm-ld"))
        }
        Err(_) => (PathBuf::from("llc"), PathBuf::from("wasm-ld")),
    };

    let temp_dir = std::env::temp_dir();
    let stem = output_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("naldom_out"));

    let ll_path = temp_dir.join(format!("{}.ll", stem.to_str().unwrap()));
    fs::write(&ll_path, llvm_ir)
        .map_err(|e| format!("Failed to write LLVM IR to temp file: {}", e))?;

    let obj_path = temp_dir.join(format!("{}.o", stem.to_str().unwrap()));

    let opt_flag = format!("-O{}", opt_level); // Optimization flag for llc

    let llc_output = Command::new(&llc_path)
        .arg(&opt_flag) // Apply optimization level
        .arg("-march=wasm32")
        .arg("-filetype=obj")
        .arg(&ll_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute '{}'. Is LLVM installed and configured correctly? Error: {}",
                llc_path.display(),
                e
            )
        })?;

    if !llc_output.status.success() {
        return Err(format!(
            "llc failed:\n{}",
            String::from_utf8_lossy(&llc_output.stderr)
        ));
    }

    let wasm_ld_output = Command::new(&wasm_ld_path)
        .arg(&obj_path)
        .arg("-o")
        .arg(output_path)
        .arg("--no-entry")
        .arg("--export-all")
        .arg("--allow-undefined")
        .arg(&opt_flag) // Also pass optimization level to the linker
        .output()
        .map_err(|e| {
            format!(
                "Failed to execute '{}'. Is wasm-ld installed and configured correctly? Error: {}",
                wasm_ld_path.display(),
                e
            )
        })?;

    if !wasm_ld_output.status.success() {
        return Err(format!(
            "wasm-ld failed:\n{}",
            String::from_utf8_lossy(&wasm_ld_output.stderr)
        ));
    }

    let _ = fs::remove_file(&ll_path);
    let _ = fs::remove_file(&obj_path);

    Ok(())
}
