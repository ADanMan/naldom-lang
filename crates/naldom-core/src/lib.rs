// crates/naldom-core/src/lib.rs

//! The core compiler components for the Naldom language.

pub mod codegen_llvm;
pub mod codegen_python;
pub mod llm_inference;
pub mod lowering;
pub mod lowering_hl_to_ll;
pub mod parser;
