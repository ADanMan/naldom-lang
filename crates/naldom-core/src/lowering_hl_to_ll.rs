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
            LowLevelValue::Constant(LLConstant::I64(*val))
        }
        HLExpression::Literal(HLValue::String(val)) => {
            // A real implementation would store the string in memory and pass a pointer.
            // For now, we convert common string commands to integer codes.
            // 0 for "ascending", 1 for "descending". Other strings are not yet supported.
            let val_as_int = match val.to_lowercase().as_str() {
                "ascending" => 0,
                "descending" => 1,
                _ => unimplemented!("String literal '{}' is not yet supported", val),
            };
            LowLevelValue::Constant(LLConstant::I64(val_as_int))
        }
        // Other cases are not yet supported as arguments.
        _ => unimplemented!("Expression type not yet supported as argument"),
    }
}

// Unit tests for the lowering pass.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowering_simple_program() {
        // 1. Arrange: Create a mock HLProgram
        let hl_program = HLProgram {
            statements: vec![
                HLStatement::Assign {
                    variable: "var_0".to_string(),
                    expression: HLExpression::FunctionCall {
                        function: "create_random_array".to_string(),
                        arguments: vec![HLExpression::Literal(HLValue::Integer(10))],
                    },
                },
                HLStatement::Call {
                    function: "sort_array".to_string(),
                    arguments: vec![
                        HLExpression::Variable("var_0".to_string()),
                        HLExpression::Literal(HLValue::String("ascending".to_string())),
                    ],
                },
                HLStatement::Call {
                    function: "print_array".to_string(),
                    arguments: vec![HLExpression::Variable("var_0".to_string())],
                },
            ],
        };

        // 2. Act: Run the lowering function
        let ll_program = lower_hl_to_ll(&hl_program);

        // 3. Assert: Check the structure of the output LLProgram
        assert_eq!(
            ll_program.functions.len(),
            1,
            "Should contain one main function"
        );
        let main_fn = &ll_program.functions[0];
        assert_eq!(main_fn.name, "main");
        assert_eq!(
            main_fn.basic_blocks.len(),
            1,
            "Main function should have one basic block"
        );

        let instructions = &main_fn.basic_blocks[0].instructions;
        assert_eq!(instructions.len(), 3, "Should have three call instructions");

        // Check the first call (create_random_array)
        if let LLInstruction::Call {
            dest,
            function_name,
            arguments,
        } = &instructions[0]
        {
            assert!(
                dest.is_some(),
                "CreateArray call should have a destination register"
            );
            assert_eq!(*function_name, "create_random_array");
            assert_eq!(arguments.len(), 1);
            assert_eq!(arguments[0], LowLevelValue::Constant(LLConstant::I64(10)));
        } else {
            panic!("First instruction was not a Call");
        }

        // Check the second call (sort_array)
        if let LLInstruction::Call {
            dest,
            function_name,
            arguments,
        } = &instructions[1]
        {
            assert!(
                dest.is_none(),
                "SortArray call should not have a destination register"
            );
            assert_eq!(*function_name, "sort_array");
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], LowLevelValue::Register(Register(0))); // Uses the result of the first call
            assert_eq!(arguments[1], LowLevelValue::Constant(LLConstant::I64(0))); // "ascending" -> 0
        } else {
            panic!("Second instruction was not a Call");
        }
    }

    #[test]
    fn test_argument_lowering() {
        // Arrange
        let mut context = LoweringContext::new();
        let reg0 = context.new_register();
        context.variable_map.insert("var_0".to_string(), reg0);

        let var_expr = HLExpression::Variable("var_0".to_string());
        let int_expr = HLExpression::Literal(HLValue::Integer(42));
        let str_expr = HLExpression::Literal(HLValue::String("descending".to_string()));

        // Act
        let var_val = lower_expression_to_value(&var_expr, &mut context);
        let int_val = lower_expression_to_value(&int_expr, &mut context);
        let str_val = lower_expression_to_value(&str_expr, &mut context);

        // Assert
        assert_eq!(var_val, LowLevelValue::Register(Register(0)));
        assert_eq!(int_val, LowLevelValue::Constant(LLConstant::I64(42)));
        assert_eq!(str_val, LowLevelValue::Constant(LLConstant::I64(1))); // "descending" -> 1
    }
}
