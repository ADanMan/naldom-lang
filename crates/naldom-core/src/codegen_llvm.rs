// crates/naldom-core/src/codegen_llvm.rs

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue};
use naldom_ir::{
    BasicBlock, LLConstant, LLFunction, LLInstruction, LLProgram, LLType, LLValue as NaldomValue,
    Register, Terminator,
};
use std::collections::HashMap;

pub struct CodeGenContext<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    registers: HashMap<Register, (PointerValue<'ctx>, LLType)>,
    #[allow(dead_code)]
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGenContext<'ctx> {
    fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        CodeGenContext {
            context,
            builder,
            module,
            registers: HashMap::new(),
            current_function: None,
        }
    }

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

    fn codegen_basic_block(&mut self, block: &BasicBlock) {
        for instr in &block.instructions {
            self.codegen_instruction(instr);
        }
        self.codegen_terminator(&block.terminator);
    }

    fn codegen_instruction(&mut self, instr: &LLInstruction) {
        match instr {
            LLInstruction::Alloc { dest, ty } => {
                let llvm_type = self.to_llvm_type(ty);
                let alloca = self
                    .builder
                    .build_alloca(llvm_type, &format!("reg_{}", dest.0))
                    .unwrap();
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

                    let dest_ptr = self
                        .builder
                        .build_alloca(return_type, &format!("reg_{}", dest_reg.0))
                        .unwrap();
                    let naldom_return_type = self.inkwell_type_to_naldom_type(return_type);
                    self.registers
                        .insert(*dest_reg, (dest_ptr, naldom_return_type));
                    self.builder.build_store(dest_ptr, return_value).unwrap();
                }
            }
            _ => unimplemented!("Instruction not yet supported in codegen"),
        }
    }

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

    fn codegen_value(&self, val: &NaldomValue) -> BasicValueEnum<'ctx> {
        match val {
            NaldomValue::Constant(c) => match c {
                LLConstant::I64(i) => self.context.i64_type().const_int(*i as u64, false).into(),
                LLConstant::I32(i) => self.context.i32_type().const_int(*i as u64, false).into(),
                LLConstant::F64(f) => self.context.f64_type().const_float(*f).into(),
            },
            NaldomValue::Register(reg) => {
                let (ptr, ty) = self.registers.get(reg).expect("Register not allocated");
                let llvm_type = self.to_llvm_type(ty);
                self.builder
                    .build_load(llvm_type, *ptr, &format!("load_reg_{}", reg.0))
                    .unwrap()
            }
        }
    }

    fn to_llvm_type(&self, ty: &LLType) -> BasicTypeEnum<'ctx> {
        match ty {
            LLType::I32 => self.context.i32_type().into(),
            LLType::I64 => self.context.i64_type().into(),
            LLType::F64 => self.context.f64_type().into(),
            LLType::Pointer(_) => self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
            LLType::Void => panic!("Cannot convert Void to a BasicTypeEnum"),
        }
    }

    fn inkwell_type_to_naldom_type(&self, ty: BasicTypeEnum) -> LLType {
        match ty {
            BasicTypeEnum::IntType(i) => {
                if i.get_bit_width() == 32 {
                    LLType::I32
                } else {
                    LLType::I64
                }
            }
            BasicTypeEnum::FloatType(_) => LLType::F64,
            BasicTypeEnum::PointerType(_) => LLType::Pointer(Box::new(LLType::F64)),
            _ => unimplemented!(),
        }
    }

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

    fn declare_placeholder_function(
        &self,
        name: &str,
        args: &[NaldomValue],
        has_return: bool,
    ) -> FunctionValue<'ctx> {
        let arg_types: Vec<BasicMetadataTypeEnum> = args
            .iter()
            .map(|arg| match arg {
                NaldomValue::Constant(LLConstant::I64(_)) => self.context.i64_type().into(),
                NaldomValue::Register(reg) => {
                    let (_, ty) = self
                        .registers
                        .get(reg)
                        .expect("Register not found during function declaration");
                    // CORRECTED: Remove the unnecessary `.clone()`
                    self.to_llvm_type(ty).into()
                }
                _ => self.context.i64_type().into(),
            })
            .collect();

        let fn_type = if has_return {
            self.context
                .ptr_type(inkwell::AddressSpace::default())
                .fn_type(&arg_types, false)
        } else {
            self.context.void_type().fn_type(&arg_types, false)
        };
        self.module.add_function(name, fn_type, None)
    }
}

pub fn generate_llvm_ir(ll_program: &LLProgram, target_triple: &str) -> Result<String, String> {
    let context = Context::create();
    let mut codegen_context = CodeGenContext::new(&context, "naldom_module");

    let triple = TargetTriple::create(target_triple);
    codegen_context.module.set_triple(&triple);

    for function in &ll_program.functions {
        codegen_context.codegen_function(function);
    }

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
