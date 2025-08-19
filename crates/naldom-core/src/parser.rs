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

#[cfg(test)]
mod tests {
    use super::*;
    use naldom_ir::Intent;

    #[test]
    fn test_parse_valid_json() {
        let llm_output = r#"
            <think>Some conversational text from the model...</think>
            [
              {
                "intent": "CreateArray",
                "parameters": {
                  "size": 10,
                  "source": "random numbers"
                }
              },
              {
                "intent": "SortArray",
                "parameters": {
                  "order": "ascending"
                }
              },
              {
                "intent": "PrintArray"
              }
            ]
        "#;

        let result = parse_to_intent_graph(llm_output);

        assert!(result.is_ok());
        let intent_graph = result.unwrap();

        assert_eq!(intent_graph.len(), 3);

        match &intent_graph[0] {
            Intent::CreateArray(params) => {
                assert_eq!(params.size, 10);
                assert_eq!(params.source, "random numbers");
            }
            _ => panic!("Expected CreateArray intent"),
        }

        match &intent_graph[1] {
            Intent::SortArray(params) => {
                assert_eq!(params.order, "ascending");
            }
            _ => panic!("Expected SortArray intent"),
        }

        assert!(matches!(intent_graph[2], Intent::PrintArray));
    }

    #[test]
    fn test_parse_invalid_json() {
        let llm_output = r#"This is not a valid JSON"#;
        let result = parse_to_intent_graph(llm_output);
        assert!(result.is_err());
    }
}
