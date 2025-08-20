// crates/naldom-core/src/lowering_hl_to_ll.rs

use naldom_ir::{
    BasicBlock, HLExpression, HLProgram, HLStatement, HLValue, LLConstant, LLFunction,
    LLInstruction, LLProgram, LLType, LLValue as LowLevelValue, Register, Terminator,
};
use std::collections::HashMap;

/// The context for the lowering process.
/// It tracks the state of the compilation for a single function.
struct LoweringContext {
    /// The next available register ID.
    next_register_id: u32,
    /// Maps high-level variable names (e.g., "var_0") to the low-level
    /// registers that hold their values.
    variable_map: HashMap<String, Register>,
    /// The instructions for the current basic block being built.
    instructions: Vec<LLInstruction>,
}

impl LoweringContext {
    /// Creates a new, empty context.
    fn new() -> Self {
        LoweringContext {
            next_register_id: 0,
            variable_map: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    /// Allocates a new virtual register.
    fn new_register(&mut self) -> Register {
        let reg = Register(self.next_register_id);
        self.next_register_id += 1;
        reg
    }
}

/// The main entry point for lowering an HLProgram to an LLProgram.
pub fn lower_hl_to_ll(hl_program: &HLProgram) -> LLProgram {
    let mut context = LoweringContext::new();

    // In the future, we will handle multiple functions. For now, we assume
    // the entire program is a single "main" function.
    for statement in &hl_program.statements {
        lower_statement(statement, &mut context);
    }

    // Create a single basic block for our simple main function.
    let main_block = BasicBlock {
        id: 0,
        instructions: context.instructions,
        // Every function must end with a return. We assume our main function returns nothing (void).
        terminator: Terminator::Return(None),
    };

    // Create the main function.
    let main_function = LLFunction {
        name: "main".to_string(),
        parameters: vec![],
        return_type: LLType::Void,
        basic_blocks: vec![main_block],
    };

    // The final LLProgram contains just our main function.
    LLProgram {
        functions: vec![main_function],
    }
}

/// Lowers a single HLStatement into one or more LLInstructions.
fn lower_statement(statement: &HLStatement, context: &mut LoweringContext) {
    match statement {
        HLStatement::Assign {
            variable,
            expression,
        } => {
            // When we see `var_0 = ...`, we first lower the expression on the right.
            // This will return the register that holds the result.
            let result_register = lower_expression(expression, context);

            // Then, we map the high-level variable name "var_0" to this register
            // so we can find it later.
            context
                .variable_map
                .insert(variable.clone(), result_register);
        }
        HLStatement::Call {
            function,
            arguments,
        } => {
            // This is a call to a function that doesn't return a value (like `print_array`).
            // We just lower it as a `Call` instruction without a destination register.
            let args = arguments
                .iter()
                .map(|arg| lower_expression_to_value(arg, context))
                .collect();

            context.instructions.push(LLInstruction::Call {
                dest: None,
                function_name: function.clone(),
                arguments: args,
            });
        }
    }
}

/// Lowers an HLExpression into a register that holds the result.
fn lower_expression(expression: &HLExpression, context: &mut LoweringContext) -> Register {
    match expression {
        HLExpression::FunctionCall {
            function,
            arguments,
        } => {
            // This is a call to a function that returns a value (like `create_random_array`).
            let args = arguments
                .iter()
                .map(|arg| lower_expression_to_value(arg, context))
                .collect();

            // We need a new register to store the return value of the function.
            let dest_register = context.new_register();

            context.instructions.push(LLInstruction::Call {
                dest: Some(dest_register),
                function_name: function.clone(),
                arguments: args,
            });

            dest_register
        }
        // Other cases will be handled later. For now, we only support function calls
        // on the right side of an assignment.
        _ => unimplemented!("Expression type not yet supported for lowering"),
    }
}

/// Lowers an HLExpression into an LLValue, which can be either a register or a constant.
/// This is used for function arguments.
fn lower_expression_to_value(
    expression: &HLExpression,
    context: &mut LoweringContext,
) -> LowLevelValue {
    match expression {
        HLExpression::Variable(name) => {
            // If an argument is a variable, we look up which register it's stored in.
            let register = context
                .variable_map
                .get(name)
                .expect("Variable not found! This indicates a logic error before lowering.");
            LowLevelValue::Register(*register)
        }
        HLExpression::Literal(HLValue::Integer(val)) => {
            // If an argument is a literal integer, we turn it into a constant.
            // We assume i64 for now.
            LowLevelValue::Constant(LLConstant::I64(*val))
        }
        HLExpression::Literal(HLValue::String(val)) => {
            // String literals are more complex. For now, we'll represent them as i64 constants
            // as a placeholder. This will be properly implemented with memory management later.
            // A real implementation would store the string in memory and pass a pointer.
            // For the prototype, this is a simplification.
            // Let's use a simple hash or a placeholder value.
            // For `sort_array('ascending')`, we can just pass 0 for ascending, 1 for descending.
            let val_as_int = if val.to_lowercase() == "ascending" {
                0
            } else {
                1
            };
            LowLevelValue::Constant(LLConstant::I64(val_as_int))
        }
        // Other cases are not yet supported as arguments.
        _ => unimplemented!("Expression type not yet supported as argument"),
    }
}
