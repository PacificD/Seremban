use dotenv::from_path;
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

    let response_body: Value = response.json().await?;
    if let Some(candidates) = response_body.get("candidates") {
        if let Some(content) = candidates[0].get("content") {
            if let Some(parts) = content.get("parts") {
                if let Some(text) = parts[0].get("text") {
                    println!("{text}");
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Parameter error!");
    }

    let gemini_target = format!(
        "{}/v1beta/models/gemini-pro:generateContent?key={}",
        &args[1], // token,
        &args[2]  // proxy
                  // &args[3] prompt
    );
    // Get the user input
    // let mut user_input = String::new();
    // println!("Enter the prompt: {gemini_target}");
    // io::stdin().read_line(&mut user_input);

    let json_string = r#"{"contents":[{"parts":[{"text":""}]}]}"#;
    // Parse the JSON string into a serde_json::Value
    let mut value: Value = from_str(json_string).unwrap();
    value["contents"][0]["parts"][0]["text"] = json!(&args[3]);
    // Convert the modified value back to a JSON string
    let prompt = to_string(&value).unwrap();

    post_gemini(gemini_target, prompt).await?;
    Ok(())
}
