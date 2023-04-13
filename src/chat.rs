use reqwest::Client;
use serde::Serialize;
use serde::Deserialize;
use serde_json; 
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

pub async fn gpt_request_user(api_key: &str, content: &str) -> Option<Vec<String>> {
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let client = Client::new();

    let request = Request {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: content.to_string(),
        }],
        temperature: 0.7,
    };

    let res = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await;

    let body = res.expect("request failed").text().await;
    let body: Value = serde_json::from_str(&body.unwrap() as &str).unwrap();

    let mut responces: Vec<String> = vec![];

    for data in body["choices"].as_array().unwrap() {
        responces.push(data["message"]["content"].to_string());
    }
    Some(responces)
}

pub async fn gpt_request_system(api_key: &str, content: &str) -> Option<String> {
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let client = Client::new();

    let request = Request {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "system".to_string(),
            content: content.to_string(),
        }],
        temperature: 0.7,
    };

    let res = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await;

    let body = res.expect("request failed").text().await;
    let body: Value = serde_json::from_str(&body.unwrap() as &str).unwrap();
    Some(body["choices"][0].to_string())
}
