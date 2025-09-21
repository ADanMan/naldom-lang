// crates/naldom-core/src/llm_inference.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

const LLM_SERVER_URL: &str = "http://127.0.0.1:8080/completion";

#[derive(Serialize)]
struct LlmRequest {
    prompt: String,
    n_predict: i32,
    temperature: f32,
    stop: Vec<String>,
    grammar: String,
}

#[derive(Deserialize)]
struct LlmResponse {
    content: String,
}

/// Runs inference against the local llama.cpp server asynchronously.
pub async fn run_inference(user_prompt: &str) -> Result<String, String> {
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
- You MUST NOT generate an intent that operates on a variable before it has been created.

DEFAULT VALUES:
- For the "SortArray" intent, if the order is not specified, you MUST default to "ascending".

AVAILABLE INTENTS (JSON Schema):
[
    {
        "intent": "CreateArray",
        "parameters": { "size": "u32" } // Updated: removed "source"
    },
    {
        "intent": "SortArray",
        "parameters": { "order": "String" }
    },
    {
        "intent": "PrintArray"
    },
    {
        "intent": "Wait",
        "parameters": { "durationMs": "u64" }
    }
]

USER REQUEST:
"#;

    let full_prompt = format!("{}{}", system_prompt, user_prompt);

    let grammar = r#"
root   ::= "[" ws intent ("," ws intent)* ws "]"
intent ::= "{" ws "\"intent\"" ws ":" ws "\"" intent-name "\"" ("," ws "\"parameters\"" ws ":" ws params)? ws "}"
params ::= "{" ws param ("," ws param)* ws "}"
param  ::= "\"" string "\"" ws ":" ws value
value  ::= string-literal | number
string-literal ::= "\"" string "\""

intent-name ::= "CreateArray" | "SortArray" | "PrintArray" | "Wait"

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

    let client = Client::new();
    let response = client
        .post(LLM_SERVER_URL)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to LLM server: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Could not retrieve response body".to_string());
        return Err(format!(
            "LLM server returned an error ({}):\n{}",
            status, body
        ));
    }

    let llm_response = response
        .json::<LlmResponse>()
        .await
        .map_err(|e| format!("Failed to parse JSON response from LLM server: {}", e))?;

    let content = llm_response.content.trim().to_string();

    println!("\nInference finished successfully.");
    Ok(content)
}
