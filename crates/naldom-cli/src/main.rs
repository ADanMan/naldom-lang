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
use std::process::{Command, Stdio};

/// The Naldom Compiler CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file_path: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(long, default_value = "native")]
    target: String,
    #[arg(short = 'O', long, default_value = "0")]
    opt_level: u8,
    #[arg(long)]
    trace: bool,
    #[arg(long)]
    run: bool,
    #[arg(long, value_name = "FORMAT")]
    emit: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    naldom_runtime::ensure_linked();

    let args = Args::parse();
    let output_path = args.output.clone().unwrap_or_else(|| {
        if args.target == "wasm" {
            PathBuf::from("a.out.wasm")
        } else {
            PathBuf::from("a.out")
        }
    });

    let llvm_ir = run_compiler_pipeline(&args).await?;

    if let Some(emit_format) = &args.emit
        && emit_format == "llvm-ir"
    {
        println!("{}", llvm_ir);
        return Ok(());
    }

    let compile_result = if args.target == "wasm" {
        compile_wasm(&llvm_ir, &output_path, args.opt_level)
    } else {
        compile_native(&llvm_ir, &output_path, args.opt_level)
    };

    if let Err(e) = compile_result {
        return Err(format!("Failed to compile for target '{}': {}", args.target, e).into());
    }

    println!("Successfully compiled to '{}'", output_path.display());

    if args.run {
        if args.target == "wasm" {
            println!(
                "\nCannot run wasm target directly. Use a Wasm runtime like wasmtime or a browser."
            );
        } else {
            run_native_executable(&output_path)?;
        }
    }

    Ok(())
}

async fn run_compiler_pipeline(args: &Args) -> Result<String, String> {
    let source_code = fs::read_to_string(&args.file_path)
        .map_err(|e| format!("Error reading file '{}': {}", args.file_path.display(), e))?;

    let llm_response = run_inference(&source_code).await?;

    let intent_graph = parse_to_intent_graph(&llm_response).map_err(|e| {
        format!(
            "Error parsing LLM response into IntentGraph: {}\n--- LLM Response ---\n{}\n--------------------",
            e, llm_response
        )
    })?;
    if args.trace {
        println!("\n... IntentGraph (Parsed) ...\n{:#?}", intent_graph);
    }
    let mut analyzer = SemanticAnalyzer::new();
    let validated_intent_graph = analyzer.analyze(&intent_graph)?;
    if args.trace {
        println!(
            "\n... IntentGraph (Validated) ...\n{:#?}",
            validated_intent_graph
        );
    }
    let mut lowering_context = LoweringContext::new();
    let hl_program = lowering_context.lower(&validated_intent_graph);
    if args.trace {
        println!("\n... High-Level IR ...\n{:#?}", hl_program);
    }
    let ll_program = lower_hl_to_ll(&hl_program);
    if args.trace {
        println!("\n... Low-Level IR ...\n{:#?}", ll_program);
    }
    let target_triple_string = if args.target == "wasm" {
        "wasm32-unknown-unknown".to_string()
    } else {
        inkwell::targets::TargetMachine::get_default_triple()
            .as_str()
            .to_str()
            .unwrap()
            .to_string()
    };
    generate_llvm_ir(&ll_program, &target_triple_string)
}

fn run_native_executable(executable_path: &Path) -> Result<(), std::io::Error> {
    println!("\nRunning '{}'...\n", executable_path.display());
    let mut command_path = PathBuf::from("./");
    command_path.push(executable_path);

    // Instead of capturing output, we inherit the stdio handles.
    // This connects the child process's output directly to our terminal,
    // which fixes the buffering issue and allows us to see output in real-time.
    let status = Command::new(&command_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?; // Use .status() instead of .output()

    if !status.success() {
        eprintln!(
            "\n❌ Program exited with non-zero status: {}",
            status.code().unwrap_or(1)
        );
    }

    Ok(())
}

fn compile_native(llvm_ir: &str, output_path: &Path, opt_level: u8) -> Result<(), String> {
    let (llc_path, clang_path) = match env::var("LLVM_PREFIX") {
        Ok(prefix) => {
            let llvm_path = PathBuf::from(prefix);
            (llvm_path.join("bin/llc"), llvm_path.join("bin/clang"))
        }
        Err(_) => (PathBuf::from("llc"), PathBuf::from("clang")),
    };
    let temp_dir = std::env::temp_dir();
    let stem = output_path.file_stem().unwrap().to_str().unwrap();
    let ll_path = temp_dir.join(format!("{}.ll", stem));
    fs::write(&ll_path, llvm_ir).map_err(|e| e.to_string())?;
    let obj_path = temp_dir.join(format!("{}.o", stem));
    let opt_flag = format!("-O{}", opt_level);
    let llc_output = Command::new(&llc_path)
        .arg(&opt_flag)
        .arg("-filetype=obj")
        .arg(&ll_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
        .map_err(|e| e.to_string())?;
    if !llc_output.status.success() {
        return Err(String::from_utf8_lossy(&llc_output.stderr).to_string());
    }
    let runtime_path = "runtime/native/naldom_runtime.c";

    let linker_path = if cfg!(debug_assertions) {
        "target/debug"
    } else {
        "target/release"
    };

    let clang_output = Command::new(&clang_path)
        .arg(&obj_path)
        .arg(runtime_path)
        .arg("-L")
        .arg(linker_path)
        .arg("-lnaldom_runtime")
        .arg("-o")
        .arg(output_path)
        .arg(&opt_flag)
        .output()
        .map_err(|e| e.to_string())?;

    if !clang_output.status.success() {
        return Err(String::from_utf8_lossy(&clang_output.stderr).to_string());
    }
    let _ = fs::remove_file(&ll_path);
    let _ = fs::remove_file(&obj_path);
    Ok(())
}

fn compile_wasm(llvm_ir: &str, output_path: &Path, opt_level: u8) -> Result<(), String> {
    let (llc_path, wasm_ld_path) = match env::var("LLVM_PREFIX") {
        Ok(prefix) => {
            let llvm_path = PathBuf::from(prefix);
            (llvm_path.join("bin/llc"), llvm_path.join("bin/wasm-ld"))
        }
        Err(_) => (PathBuf::from("llc"), PathBuf::from("wasm-ld")),
    };
    let temp_dir = std::env::temp_dir();
    let stem = output_path.file_stem().unwrap().to_str().unwrap();
    let ll_path = temp_dir.join(format!("{}.ll", stem));
    fs::write(&ll_path, llvm_ir).map_err(|e| e.to_string())?;
    let obj_path = temp_dir.join(format!("{}.o", stem));
    let opt_flag = format!("-O{}", opt_level);
    let llc_output = Command::new(&llc_path)
        .arg(&opt_flag)
        .arg("-march=wasm32")
        .arg("-filetype=obj")
        .arg(&ll_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
        .map_err(|e| e.to_string())?;
    if !llc_output.status.success() {
        return Err(String::from_utf8_lossy(&llc_output.stderr).to_string());
    }
    let wasm_ld_output = Command::new(&wasm_ld_path)
        .arg(&obj_path)
        .arg("-o")
        .arg(output_path)
        .arg("--no-entry")
        .arg("--export-all")
        .arg("--allow-undefined")
        .arg(&opt_flag)
        .output()
        .map_err(|e| e.to_string())?;
    if !wasm_ld_output.status.success() {
        return Err(String::from_utf8_lossy(&wasm_ld_output.stderr).to_string());
    }
    let _ = fs::remove_file(&ll_path);
    let _ = fs::remove_file(&obj_path);
    Ok(())
}
