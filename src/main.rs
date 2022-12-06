use std::env;
use std::error::Error;

use reqwest;
use structopt::StructOpt;
use serde_json::{self, json};

// Define a result type for the API response
type OpenAIResult = Result<serde_json::Value, Box<dyn Error>>;

// Define global variables for the API endpoint and model
const API_ENDPOINT: &str = "https://api.openai.com/v1/completions";

// Define the Client struct
pub struct Client {
    api_key: String,
}

impl Client {
    // Implement the new method
    pub fn new() -> Result<Client, Box<dyn std::error::Error>> {
        // Read the API key from environment variables
        let api_key = env::var("OPENAI_API_KEY")?;
        Ok(Client{ api_key })
    }

    // Implement the prompt method
    pub async fn prompt(&self, request_body: &serde_json::Value) -> OpenAIResult {
        // Build the request URL with the model and version
        let url = String::from(API_ENDPOINT);
    
        // Convert the URL to a `Url` type and send the request
        let url: reqwest::Url = url.parse()?;
        let client = reqwest::Client::new();
        let response: serde_json::Value = client.post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request_body)
            .send()
            .await?
            .json()
            .await?;
    
        // Return the response
        Ok(response)
    }
}

#[derive(StructOpt, Debug)]
struct Cli {
    // The prompt to generate text for
    #[structopt(short = "p", long = "prompt")]
    prompt: String,

    // The model to use for text generation
    #[structopt(short = "m", long = "model")]
    model: String,

    // The temperature to use for text generation
    #[structopt(short = "t", long = "temperature")]
    temperature: f64,

    // The maximum number of tokens to generate
    #[structopt(short = "x", long = "max_tokens")]
    max_tokens: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let args = Cli::from_args();

    // Create a new client instance
    let client = Client::new()?;

    // Build the request body
    let request_body = json!({
        "prompt": &args.prompt,
        "model": &args.model,
        "temperature": args.temperature,
        "max_tokens": args.max_tokens,
    });

    // Generate text using the prompt
    let response = client.prompt(&request_body).await?;

    // Print the response
    println!("{}", response);

    Ok(())
}