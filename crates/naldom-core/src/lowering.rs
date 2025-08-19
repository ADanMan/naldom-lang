// crates/naldom-core/src/lowering.rs

use naldom_ir::*;

/// A stateful struct that handles the lowering process from IntentGraph to IR-HL.
/// It keeps track of generated variables to chain operations together.
pub struct LoweringContext {
    variable_counter: u32,
    last_created_variable: Option<String>,
}

// Implement the `Default` trait as suggested by Clippy.
// This allows creating a new, empty context with `LoweringContext::default()`.
impl Default for LoweringContext {
    fn default() -> Self {
        Self::new()
    }
}

impl LoweringContext {
    /// Creates a new, empty lowering context.
    pub fn new() -> Self {
        LoweringContext {
            variable_counter: 0,
            last_created_variable: None,
        }
    }

    /// Generates a new, unique variable name (e.g., "var_0", "var_1").
    fn new_variable_name(&mut self) -> String {
        let name = format!("var_{}", self.variable_counter);
        self.variable_counter += 1;
        name
    }

    /// The main function that transforms a sequence of intents into an HLProgram.
    pub fn lower(&mut self, intent_graph: &[Intent]) -> HLProgram {
        let mut statements = Vec::new();

        for intent in intent_graph {
            match intent {
                Intent::CreateArray(params) => {
                    let new_var = self.new_variable_name();
                    statements.push(HLStatement::Assign {
                        variable: new_var.clone(),
                        expression: HLExpression::FunctionCall {
                            function: FUNC_CREATE_RANDOM_ARRAY.to_string(),
                            arguments: vec![HLExpression::Literal(HLValue::Integer(
                                params.size as i64,
                            ))],
                        },
                    });
                    self.last_created_variable = Some(new_var);
                }
                Intent::SortArray(params) => {
                    if let Some(var_to_sort) = &self.last_created_variable {
                        statements.push(HLStatement::Call {
                            function: FUNC_SORT_ARRAY.to_string(),
                            arguments: vec![
                                HLExpression::Variable(var_to_sort.clone()),
                                HLExpression::Literal(HLValue::String(params.order.clone())),
                            ],
                        });
                    }
                    // TODO: Handle the case where there is no variable to sort (error).
                }
                Intent::PrintArray => {
                    if let Some(var_to_print) = &self.last_created_variable {
                        statements.push(HLStatement::Call {
                            function: FUNC_PRINT_ARRAY.to_string(),
                            arguments: vec![HLExpression::Variable(var_to_print.clone())],
                        });
                    }
                    // TODO: Handle the case where there is no variable to print (error).
                }
            }
        }

        HLProgram { statements }
    }
}

// Constants for function names to avoid magic strings.
const FUNC_CREATE_RANDOM_ARRAY: &str = "create_random_array";
const FUNC_SORT_ARRAY: &str = "sort_array";
const FUNC_PRINT_ARRAY: &str = "print_array";
