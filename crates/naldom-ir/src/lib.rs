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
