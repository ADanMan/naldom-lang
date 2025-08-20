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

/// Low-Level Intermediate Representation (IR-LL).
///
/// This is a much lower-level, explicit representation, very close to LLVM IR or assembly.
/// It operates on concepts like virtual registers, basic blocks, and simple, atomic instructions.
/// This representation is the final step before generating target-specific code (like LLVM IR).
#[derive(Debug, Clone, PartialEq)]
pub struct LLProgram {
    pub functions: Vec<LLFunction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LLFunction {
    pub name: String,
    pub parameters: Vec<(LLType, Register)>,
    pub return_type: LLType,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub id: usize,
    pub instructions: Vec<LLInstruction>,
    pub terminator: Terminator,
}

/// A virtual register, representing a temporary value. e.g., `%0`, `%1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(pub u32);

/// Represents the primitive types in our low-level language.
#[derive(Debug, Clone, PartialEq)]
pub enum LLType {
    Void,
    I32,
    I64,
    F64,
    Pointer(Box<LLType>),
}

/// Represents a single, atomic operation.
#[derive(Debug, Clone, PartialEq)]
pub enum LLInstruction {
    /// Allocates space on the stack. Returns a pointer to the allocated space.
    Alloc { dest: Register, ty: LLType },
    /// Loads a value from a memory address (pointer).
    Load {
        dest: Register,
        source_ptr: Register,
    },
    /// Stores a value to a memory address (pointer).
    Store { value: LLValue, dest_ptr: Register },
    /// Calls a function.
    Call {
        dest: Option<Register>, // `None` for void functions
        function_name: String,
        arguments: Vec<LLValue>,
    },
    // We will add more instructions like Add, Sub, ICmp later.
}

/// Represents an instruction that terminates a basic block, controlling flow.
#[derive(Debug, Clone, PartialEq)]
pub enum Terminator {
    /// Returns from a function.
    Return(Option<LLValue>),
    // We will add branching instructions like `Br` and `CondBr` later.
}

/// Represents a value that can be used as an operand in an instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum LLValue {
    Register(Register),
    Constant(LLConstant),
}

/// Represents a constant literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum LLConstant {
    I32(i32),
    I64(i64),
    F64(f64),
    // We can add string literals, etc., later.
}
