// crates/naldom-core/src/llm_inference.rs

use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;

const LLM_SERVER_URL: &str = "http://127.0.0.1:8080/completion";

#[derive(Serialize)]
struct LlmRequest<'a> {
    prompt: &'a str,
    n_predict: i32,
    temperature: f32,
    stop: Vec<&'a str>,
    grammar: &'a str,
}

/// Runs inference against the local llama.cpp server.
pub fn run_inference(user_prompt: &str) -> Result<String, String> {
    // NEW: Structured prompt based on prompt engineering best practices.
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

    // This grammar ensures the LLM *must* produce JSON that fits our structure.
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
        prompt: &full_prompt,
        n_predict: 512,
        temperature: 0.1,
        stop: vec!["\nUSER REQUEST:", "ASSISTANT:"],
        grammar,
    };

    println!("Sending HTTP request to llama.cpp server...");

    let client = Client::new();
    let response = client
        .post(LLM_SERVER_URL)
        .json(&request_body)
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "LLM server returned an error: {}",
            response.status()
        ));
    }

    let response_json: Value = response.json().map_err(|e| e.to_string())?;
    let content = response_json["content"]
        .as_str()
        .ok_or("Invalid response format: 'content' field is not a string")?
        .trim() // Trim whitespace which can sometimes be added by the model
        .to_string();

    println!("\nInference finished successfully.");
    Ok(content)
}
