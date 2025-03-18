use reqwest::Client;
use dotenvy::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize, Debug)]
struct MessageContent {
    content: String,
}

pub async fn get_build(champion: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = load_api_key();
    let payload = create_payload(champion);
    let response_text = send_request(&api_key, payload).await?;
    let chat_response = parse_response(&response_text)?;
    extract_build_content(chat_response)
}

fn load_api_key() -> String {
    dotenv().ok();
    env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY not set")
}

fn create_payload(champion: &str) -> serde_json::Value {
    let agent_id = "ag:b04d0054:20250318:untitled-agent:6536c74c";
    let max_tokens = 150;

    serde_json::json!({
        "agent_id": agent_id,
        "messages": [{"role": "user", "content": champion}],
        "max_tokens": max_tokens
    })
}

/// calling mistral agent API, with temperature 0.2 and model NeMo, for more info check the README.md
async fn send_request(api_key: &str, payload: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .post("https://api.mistral.ai/v1/agents/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    response.text().await.map_err(|e| e.into())
}

fn parse_response(response_text: &str) -> Result<ChatResponse, Box<dyn std::error::Error>> {
    serde_json::from_str(response_text).map_err(|e| e.into())
}

fn extract_build_content(chat_response: ChatResponse) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(choice) = chat_response.choices.get(0) {
        if choice.message.content.contains("BUILD") {
            Ok(choice.message.content.clone())
        } else {
            Ok("build not found, sorry =(".to_string())
        }
    } else {
        Err("Error: No choices found".into())
    }
}