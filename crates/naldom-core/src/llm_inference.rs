// crates/naldom-core/src/llm_inference.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

const LLM_SERVER_URL: &str = "http://127.0.0.1:8080/completion";

// Using owned String types is easier to manage in an async context
#[derive(Serialize)]
struct LlmRequest {
    prompt: String,
    n_predict: i32,
    temperature: f32,
    stop: Vec<String>,
    grammar: String,
}

// A dedicated struct for the response is cleaner and safer than using serde_json::Value
#[derive(Deserialize)]
struct LlmResponse {
    content: String,
}

/// Runs inference against the local llama.cpp server asynchronously.
pub async fn run_inference(user_prompt: &str) -> Result<String, String> {
    // The detailed system prompt is preserved
    let system_prompt = r#"
CONTEXT:
You are an expert Frontend Compiler. Your task is to analyze the user's request, which is written in a natural language called Naldom, and transform it into a strictly structured JSON array of "intents". This JSON is the Abstract Syntax Tree (AST) for the Naldom language.

TASK:
1. Analyze the user's request.
2. Identify the sequence of operations the user wants to perform.
3. For each operation, map it to one of the "AVAILABLE INTENTS".
4. Construct a JSON object for each intent.
5. Combine these objects into a single JSON array.
6. Respond with ONLY the raw JSON array.

IMPORTANT:
- You MUST respond with ONLY the JSON array. Do not include any extra text, explanations, markdown formatting, or "think" blocks.
- If a parameter is not specified by the user, you MUST use a sensible default value as specified below.
- You MUST NOT generate an intent that operates on a variable before it has been created. For example, a "SortArray" intent cannot be the first intent in the array unless a variable is already in context (which is not supported yet).

DEFAULT VALUES:
- For the "SortArray" intent, if the order is not specified, you MUST default to "ascending".

AVAILABLE INTENTS (JSON Schema):
[
    {
        "intent": "CreateArray",
        "parameters": { "size": "u32", "source": "String" }
    },
    {
        "intent": "SortArray",
        "parameters": { "order": "String" }
    },
    {
        "intent": "PrintArray" // This intent has no parameters.
    }
]

USER REQUEST:
"#;

    let full_prompt = format!("{}{}", system_prompt, user_prompt);

    // The grammar to enforce JSON output is preserved
    let grammar = r#"
root   ::= "[" ws intent ("," ws intent)* ws "]"
intent ::= "{" ws "\"intent\"" ws ":" ws "\"" intent-name "\"" ("," ws "\"parameters\"" ws ":" ws params)? ws "}"
params ::= "{" ws param ("," ws param)* ws "}"
param  ::= "\"" string "\"" ws ":" ws value
value  ::= string-literal | number
string-literal ::= "\"" string "\""

intent-name ::= "CreateArray" | "SortArray" | "PrintArray"
string ::= ([^"\\] | "\\" (["\\/bfnrt] | "u" [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F]))*
number ::= "-"? ([0-9] | [1-9] [0-9]*) ("." [0-9]+)? ([eE] [-+]? [0-9]+)?
ws ::= [ \t\n\r]*
"#;

    let request_body = LlmRequest {
        prompt: full_prompt,
        n_predict: 512,
        temperature: 0.1,
        stop: vec!["\nUSER REQUEST:".to_string(), "ASSISTANT:".to_string()],
        grammar: grammar.to_string(),
    };

    println!("Sending HTTP request to llama.cpp server...");

    // Use the async client
    let client = Client::new();
    let response = client
        .post(LLM_SERVER_URL)
        .json(&request_body)
        .send()
        .await // Await the async send operation
        .map_err(|e| format!("Failed to send request to LLM server: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await // Await the async text retrieval
            .unwrap_or_else(|_| "Could not retrieve response body".to_string());
        return Err(format!(
            "LLM server returned an error ({}):\n{}",
            status, body
        ));
    }

    // Await the async JSON parsing
    let llm_response = response
        .json::<LlmResponse>()
        .await
        .map_err(|e| format!("Failed to parse JSON response from LLM server: {}", e))?;

    let content = llm_response.content.trim().to_string();

    println!("\nInference finished successfully.");
    Ok(content)
}
