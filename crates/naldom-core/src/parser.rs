// crates/naldom-core/src/parser.rs

use naldom_ir::Intent;
use serde_json;

pub fn parse_to_intent_graph(llm_output: &str) -> Result<Vec<Intent>, serde_json::Error> {
    // A robust method to find and extract the JSON array part of the string.
    let json_part = if let Some(start_index) = llm_output.find('[') {
        // If we found a start bracket, find the corresponding end bracket starting from that point.
        if let Some(end_index) = llm_output[start_index..].rfind(']') {
            // The slice is relative to the start_index, so we need to adjust it.
            &llm_output[start_index..start_index + end_index + 1]
        } else {
            // A start bracket was found, but no end bracket.
            // Pass the potentially malformed string to serde_json to handle the error.
            llm_output
        }
    } else {
        // No start bracket found at all.
        // Pass the whole string to serde_json to handle the error.
        llm_output
    };

    serde_json::from_str(json_part.trim())
}
