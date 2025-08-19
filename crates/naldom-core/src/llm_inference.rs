// crates/naldom-core/src/llm_inference.rs

use serde::{Deserialize, Serialize};

const LLAMA_SERVER_URL: &str = "http://127.0.0.1:8080/completion";
const DETERMINISTIC_SEED: i64 = 42;

// Structures for serializing the request and deserializing the response.
#[derive(Serialize)]
struct CompletionRequest<'a> {
    prompt: &'a str,
    n_predict: i32,
    temperature: f32,
    top_k: i32,
    top_p: f32,
    presence_penalty: f32,
    repeat_penalty: f32,
    seed: i64,
    stop: Vec<&'static str>,
}

#[derive(Deserialize)]
struct CompletionResponse {
    content: String,
}

pub type InferenceResult = Result<String, Box<dyn std::error::Error>>;

pub fn run_inference(prompt: &str) -> InferenceResult {
    println!("Sending HTTP request to llama.cpp server...");

    // 1. Format the prompt for the Qwen3 model's chat template.
    let system_prompt = "You are an expert natural language to JSON compiler. Your task is to convert the user's request into a JSON array of intents. Do not add any explanations or markdown formatting. Only output the raw JSON.";

    let user_prompt = format!(
        "Available intents:\n- CreateArray(size: u32, source: String)\n- SortArray(order: String)\n- PrintArray()\n\nUser request:\n\"{}\"",
        prompt
    );

    let formatted_prompt = format!(
        "<|im_start|>system\n{}<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
        system_prompt, user_prompt
    );

    // 2. Create the request body with the new deterministic parameters.
    let request_body = CompletionRequest {
        prompt: &formatted_prompt,
        n_predict: 1024,
        temperature: 0.3,
        top_k: 20,
        top_p: 0.0,
        presence_penalty: 1.5,
        repeat_penalty: 1.0,
        seed: DETERMINISTIC_SEED, // Set a fixed seed for deterministic output.
        stop: vec!["<|im_end|>", "<|endoftext|>"],
    };

    // 3. Create a blocking HTTP client and send the request.
    let client = reqwest::blocking::Client::new();
    let response = client.post(LLAMA_SERVER_URL).json(&request_body).send()?;

    // 4. Check the response status and parse the JSON.
    if response.status().is_success() {
        let completion_response = response.json::<CompletionResponse>()?;
        println!("\nInference finished successfully.");
        // Return the "content" field from the server's response.
        Ok(completion_response.content)
    } else {
        // If the server returned an error, display it.
        let error_body = response.text()?;
        Err(format!("Server returned an error: {}", error_body).into())
    }
}
