// crates/naldom-core/src/codegen_python.rs

use naldom_ir::*;

/// A struct responsible for generating Python code from IR-HL.
pub struct PythonCodeGenerator;

impl PythonCodeGenerator {
    pub fn new() -> Self {
        Self
    }

    /// The main entry point for code generation.
    pub fn generate(&self, program: &HLProgram) -> String {
        let mut output = Vec::new();
        for statement in &program.statements {
            output.push(self.generate_statement(statement));
        }
        output.join("\n")
    }

    /// Generates a single Python statement.
    fn generate_statement(&self, statement: &HLStatement) -> String {
        match statement {
            HLStatement::Assign {
                variable,
                expression,
            } => {
                format!("{} = {}", variable, self.generate_expression(expression))
            }
            HLStatement::Call {
                function,
                arguments,
            } => {
                let args_str = arguments
                    .iter()
                    .map(|arg| self.generate_expression(arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", function, args_str)
            }
        }
    }

    /// Generates code for an expression.
    fn generate_expression(&self, expression: &HLExpression) -> String {
        match expression {
            HLExpression::Literal(value) => self.generate_value(value),
            HLExpression::Variable(name) => name.clone(),
            HLExpression::FunctionCall {
                function,
                arguments,
            } => {
                let args_str = arguments
                    .iter()
                    .map(|arg| self.generate_expression(arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", function, args_str)
            }
        }
    }

    /// Generates code for a literal value.
    fn generate_value(&self, value: &HLValue) -> String {
        match value {
            HLValue::Integer(i) => i.to_string(),
            HLValue::String(s) => format!("'{}'", s), // Wrap strings in single quotes
        }
    }
}
