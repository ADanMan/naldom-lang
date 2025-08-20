// crates/naldom-core/src/codegen_llvm.rs

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue};
use naldom_ir::{
    BasicBlock, LLConstant, LLFunction, LLInstruction, LLProgram, LLType, LLValue as NaldomValue,
    Register, Terminator,
};
use std::collections::HashMap;

/// The context for LLVM code generation.
/// It holds all the necessary LLVM objects and state for the compilation of one module.
pub struct CodeGenContext<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,

    /// Maps our virtual registers to a tuple containing:
    /// 1. The LLVM pointer (`alloca`) for the register's storage.
    /// 2. Our own `LLType` to remember what type was allocated.
    registers: HashMap<Register, (PointerValue<'ctx>, LLType)>,

    /// The current function being built.
    #[allow(dead_code)] // This will be used later for more complex scenarios.
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGenContext<'ctx> {
    /// Creates a new CodeGenContext.
    fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("naldom_module");
        let builder = context.create_builder();
        CodeGenContext {
            context,
            builder,
            module,
            registers: HashMap::new(),
            current_function: None,
        }
    }

    /// The main entry point for code generation for a single function.
    fn codegen_function(&mut self, func: &LLFunction) {
        let fn_type = self.to_llvm_fn_type(&func.parameters, &func.return_type);
        let function = self.module.add_function(&func.name, fn_type, None);
        self.current_function = Some(function);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        if let Some(block) = func.basic_blocks.first() {
            self.codegen_basic_block(block);
        }
    }

    /// Generates code for a single basic block.
    fn codegen_basic_block(&mut self, block: &BasicBlock) {
        for instr in &block.instructions {
            self.codegen_instruction(instr);
        }
        self.codegen_terminator(&block.terminator);
    }

    /// Generates code for a single instruction.
    fn codegen_instruction(&mut self, instr: &LLInstruction) {
        match instr {
            LLInstruction::Alloc { dest, ty } => {
                let llvm_type = self.to_llvm_type(ty);
                let alloca = self
                    .builder
                    .build_alloca(llvm_type, &format!("reg_{}", dest.0))
                    .unwrap();
                // Store both the pointer and our original type for later lookup.
                self.registers.insert(*dest, (alloca, ty.clone()));
            }
            LLInstruction::Call {
                dest,
                function_name,
                arguments,
            } => {
                let callee = self.module.get_function(function_name).unwrap_or_else(|| {
                    self.declare_placeholder_function(function_name, arguments, dest.is_some())
                });

                let args: Vec<BasicMetadataValueEnum> = arguments
                    .iter()
                    .map(|arg| self.codegen_value(arg).into())
                    .collect();

                let call_site_value = self.builder.build_call(callee, &args, "call_tmp").unwrap();

                if let Some(dest_reg) = dest {
                    let return_value = call_site_value
                        .try_as_basic_value()
                        .left()
                        .expect("Call did not return a value");
                    let return_type = return_value.get_type();

                    // Store the return value in a new allocation for the destination register.
                    let dest_ptr = self
                        .builder
                        .build_alloca(return_type, &format!("reg_{}", dest_reg.0))
                        .unwrap();
                    // We need to figure out our own LLType from the inkwell type. This is a simplification.
                    let naldom_return_type = LLType::F64; // Assuming F64 for now.
                    self.registers
                        .insert(*dest_reg, (dest_ptr, naldom_return_type));
                    self.builder.build_store(dest_ptr, return_value).unwrap();
                }
            }
            _ => unimplemented!("Instruction not yet supported in codegen"),
        }
    }

    /// Generates code for a terminator instruction.
    fn codegen_terminator(&mut self, term: &Terminator) {
        match term {
            Terminator::Return(Some(val)) => {
                let llvm_val = self.codegen_value(val);
                self.builder.build_return(Some(&llvm_val)).unwrap();
            }
            Terminator::Return(None) => {
                self.builder.build_return(None).unwrap();
            }
        }
    }

    /// Converts our `NaldomValue` into an `inkwell::values::BasicValueEnum`.
    fn codegen_value(&self, val: &NaldomValue) -> BasicValueEnum<'ctx> {
        match val {
            NaldomValue::Constant(c) => match c {
                LLConstant::I64(i) => self.context.i64_type().const_int(*i as u64, false).into(),
                LLConstant::I32(i) => self.context.i32_type().const_int(*i as u64, false).into(),
                LLConstant::F64(f) => self.context.f64_type().const_float(*f).into(),
            },
            NaldomValue::Register(reg) => {
                // To use a register's value, we must load it from its stack allocation.
                let (ptr, ty) = self.registers.get(reg).expect("Register not allocated");
                let llvm_type = self.to_llvm_type(ty);
                self.builder
                    .build_load(llvm_type, *ptr, &format!("load_reg_{}", reg.0))
                    .unwrap()
            }
        }
    }

    /// Helper to convert our `LLType` to an `inkwell` type.
    fn to_llvm_type(&self, ty: &LLType) -> BasicTypeEnum<'ctx> {
        match ty {
            LLType::I32 => self.context.i32_type().into(),
            LLType::I64 => self.context.i64_type().into(),
            LLType::F64 => self.context.f64_type().into(),
            LLType::Pointer(_inner) => self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
            LLType::Void => panic!("Cannot convert Void to a BasicTypeEnum"),
        }
    }

    /// Helper to build an `inkwell` function type from our types.
    fn to_llvm_fn_type(
        &self,
        params: &[(LLType, Register)],
        ret: &LLType,
    ) -> inkwell::types::FunctionType<'ctx> {
        let param_types: Vec<BasicMetadataTypeEnum> = params
            .iter()
            .map(|(ty, _)| self.to_llvm_type(ty).into())
            .collect();

        match ret {
            LLType::Void => self.context.void_type().fn_type(&param_types, false),
            _ => self.to_llvm_type(ret).fn_type(&param_types, false),
        }
    }

    /// Creates a placeholder function declaration if one is not found.
    fn declare_placeholder_function(
        &self,
        name: &str,
        args: &[NaldomValue],
        has_return: bool,
    ) -> FunctionValue<'ctx> {
        let arg_types: Vec<BasicMetadataTypeEnum> = args
            .iter()
            .map(|arg| {
                match arg {
                    NaldomValue::Constant(LLConstant::I64(_)) => self.context.i64_type().into(),
                    NaldomValue::Register(reg) => {
                        let (_, ty) = self.registers.get(reg).unwrap();
                        self.to_llvm_type(ty).into()
                    }
                    _ => self.context.i64_type().into(), // Default fallback
                }
            })
            .collect();

        let fn_type = if has_return {
            self.context.f64_type().fn_type(&arg_types, false) // Assuming f64 return
        } else {
            self.context.void_type().fn_type(&arg_types, false)
        };
        self.module.add_function(name, fn_type, None)
    }
}

/// The main entry point for generating LLVM IR from an LLProgram.
pub fn generate_llvm_ir(ll_program: &LLProgram) -> Result<String, String> {
    let context = Context::create();
    let mut codegen_context = CodeGenContext::new(&context);

    for function in &ll_program.functions {
        codegen_context.codegen_function(function);
    }

    // Verify the generated module for correctness.
    if let Err(e) = codegen_context.module.verify() {
        let ir_string = codegen_context.module.print_to_string().to_string();
        return Err(format!(
            "LLVM module verification failed: {}\nGenerated IR:\n{}",
            e.to_string(),
            ir_string
        ));
    }

    Ok(codegen_context.module.print_to_string().to_string())
}
