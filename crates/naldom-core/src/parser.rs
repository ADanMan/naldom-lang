// crates/naldom-core/src/parser.rs

use naldom_ir::Intent;
use serde_json;

/// Parses a JSON string from the LLM into a structured IntentGraph.
///
/// This function is designed to be resilient: it first finds the part of the
/// string that looks like a JSON array (starting with `[` and ending with `]`)
/// to filter out any extraneous text like the LLM's `<think>` block.
pub fn parse_to_intent_graph(llm_output: &str) -> Result<Vec<Intent>, serde_json::Error> {
    // Find the start of the JSON array `[`
    let json_start = llm_output.find('[').unwrap_or(0);

    // Find the end of the JSON array `]`
    let json_end = llm_output.rfind(']').unwrap_or(llm_output.len());

    // Slice the string to get only the JSON part
    let json_part = &llm_output[json_start..=json_end];

    // Use serde_json to parse the slice into our Rust structures
    serde_json::from_str(json_part.trim())
}
