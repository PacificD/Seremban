use dotenv::dotenv;
use reqwest::{Client, Error};
use serde_json::{from_str, json, to_string, Value};
use std::{env, io};
use tokio;

async fn post_gemini(gemini_target: String, prompt: String) -> Result<(), Error> {
    let client = Client::new();

    let response = client
        .post(gemini_target)
        .header("Content-Type", "application/json")
        .body(prompt)
        .send()
        .await?;

    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = env::var("TOKEN").unwrap_or_else(|_| "token not found in .env file".to_string());
    let proxy =
        env::var("PROXY").unwrap_or_else(|_| "proxy string not found in .env file".to_string());
    let gemini_target = format!(
        "{}/v1beta/models/gemini-pro:generateContent?key={}",
        proxy, token
    );
    // Get the user input
    let mut user_input = String::new();
    println!("Enter the prompt:");
    io::stdin().read_line(&mut user_input);

    let json_string = r#"{"contents":[{"parts":[{"text":""}]}]}"#;
    // Parse the JSON string into a serde_json::Value
    let mut value: Value = from_str(json_string).unwrap();
    value["contents"][0]["parts"][0]["text"] = json!(user_input);
    // Convert the modified value back to a JSON string
    let prompt = to_string(&value).unwrap();

    post_gemini(gemini_target, prompt).await?;
    Ok(())
}
