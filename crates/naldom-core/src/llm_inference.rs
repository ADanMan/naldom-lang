// crates/naldom-core/src/llm_inference.rs

use std::path::PathBuf;
use std::convert::Infallible;
use std::io::Write;

// Define a type alias for our Result type for convenience
pub type InferenceResult = Result<String, Box<dyn std::error::Error>>;

pub fn run_inference(prompt: &str) -> InferenceResult {
    // 1. Specify the path to the model. This path is relative to the project root.
    let model_path = PathBuf::from("llm/models/Qwen3-1.7B-Q8_0.gguf");

    // 2. Load the model, specifying that we want to log progress to stdout
    let model = llm::load(
        &model_path,
        llm::TokenizerSource::Embedded,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    // 3. Create a new session for inference
    let mut session = model.start_session(Default::default());

    println!("Running inference...");

    // 4. Perform inference with the parameters you provided
    let res = session.infer(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: prompt.into(),
            parameters: &llm::InferenceParameters {
                temperature: 0.6,
                repeat_penalty: 1.0,
                top_k: 20,
                top_p: 0.95,
                ..Default::default()
            },
            play_back_previous_tokens: false,
            maximum_token_count: Some(1024), // Limit the maximum number of generated tokens
        },
        // `output_request` is used to handle the output in real-time
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::EotToken => Ok(llm::InferenceFeedback::Halt),
            llm::InferenceResponse::InferredToken(t) => {
                // Simply print the tokens to the console as they are generated
                print!("{t}");
                std::io::stdout().flush().unwrap();
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    // This part of the code will be reached after the inference is complete or has failed.
    match res {
        Ok(_) => {
            // For now, we are just printing in real-time,
            // so we'll return an empty string on success.
            // Later, we will modify this to collect the tokens into a string.
            println!("\nInference finished successfully.");
            Ok(String::new())
        }
        Err(e) => {
            eprintln!("\nInference failed: {}", e);
            Err(Box::new(e))
        }
    }
}
