// crates/naldom-ir/src/lib.rs

use serde::Deserialize;

/// Represents a single user intent, parsed from the LLM's JSON output.
/// This is the core enum for the IntentGraph.
///
/// Serde attributes explained:
/// - `#[serde(tag = "intent")]`: Tells serde that this is a "tagged" enum. The field named "intent"
///   in the JSON will determine which variant of this enum to use.
///   (e.g., `"intent": "CreateArray"` maps to `Intent::CreateArray`).
/// - `#[serde(content = "parameters")]`: Tells serde that the data for the variant (if any)
///   is located in a field named "parameters".
/// - `#[serde(rename_all = "PascalCase")]`: Automatically converts JSON's "PascalCase" names
///   (like "CreateArray") to Rust's PascalCase enum variants (like `CreateArray`).
#[derive(Debug, Deserialize)]
#[serde(tag = "intent", content = "parameters", rename_all = "PascalCase")]
pub enum Intent {
    CreateArray(CreateArrayParams),
    SortArray(SortArrayParams),
    PrintArray,
}

/// Parameters for the `CreateArray` intent.
#[derive(Debug, Deserialize)]
pub struct CreateArrayParams {
    pub size: u32,
    pub source: String,
}

/// Parameters for the `SortArray` intent.
#[derive(Debug, Deserialize)]
pub struct SortArrayParams {
    pub order: String,
}

/// High-Level Intermediate Representation (IR-HL).
///
/// This represents the program in a more traditional, abstract way, with
/// statements, expressions, and variables. It's the bridge between the
/// user's "intent" and the actual code generation.
#[derive(Debug, Clone, PartialEq)]
pub struct HLProgram {
    pub statements: Vec<HLStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HLStatement {
    /// Assigns the result of an expression to a variable.
    /// e.g., `var_0 = create_random_array(10)`
    Assign {
        variable: String,
        expression: HLExpression,
    },
    /// Calls a function that does not return a value (e.g., a procedure).
    /// e.g., `print_array(var_0)`
    Call {
        function: String,
        arguments: Vec<HLExpression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum HLExpression {
    /// A literal value, like a number or a string.
    Literal(HLValue),
    /// A reference to a variable.
    Variable(String),
    /// A call to a function that returns a value.
    /// e.g., `sort_array(var_0)` might return a new, sorted array.
    FunctionCall {
        function: String,
        arguments: Vec<HLExpression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum HLValue {
    Integer(i64),
    String(String),
    // We can add more types like Float, Bool, etc. later.
}
