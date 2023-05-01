use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Role {
    user,
    system,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GptRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

impl GptRequest {
    pub fn new(messages: Vec<Message>, temperature: f32) -> Self {
        GptRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            temperature,
        }
    }

    pub async fn send_request(
        &self,
        api_key: &str,
        raw_response: bool,
    ) -> Result<Vec<String>, GptError> {
        let endpoint = "https://api.openai.com/v1/chat/completions";

        let client = Client::new();

        let res = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(self)
            .send()
            .await;
        let body = res
            .map_err(|e| GptError::ApiError(format!("request failed: {}", e)))?
            .text()
            .await
            .map_err(|e| GptError::ApiError(format!("response error: {}", e)))?;
        let body: Value = serde_json::from_str(&body)?;
        // dbg!(&body);

        let mut responses: Vec<String> = vec![];
        for data in body["choices"].as_array().unwrap() {
            let content = &data["message"]["content"];
            if raw_response {
                responses.push(content.to_string()
                               .replace(r"\\n", r"\n")
                               .replace("\\\"", "")
                               );
            } else {
                responses.push(content.to_string().replace("\n", " "));
            }
        }

        Ok(responses)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    system_prompt: String,
    lang_specific_information: String,
    ignore_files: Vec<String>,
    function_description_length: String,
    include_overall_summary: bool,
    api_key: String,
}

impl Config {
    pub fn load_file(directory: &str) -> Result<(Self, String), Box<dyn Error>> {
        let file_path = format!("{}/auto_doc.yaml", directory);
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;
        let api_from_config: Config = serde_yaml::from_str(&contents)?;

        Ok((config, api_from_config.api_key))
    }

    pub fn config_to_system_prompt(&self) -> Result<Message, Box<dyn Error>> {
        let message = Message {
            role: Role::system,
            content: format!("{}. Addional language specific information: {}.
                             description length in the documentation: {}",
                             &self.system_prompt,
                             &self.lang_specific_information,
                             &self.function_description_length
                             ).to_string(),
        };
        Ok(message)
    }
}

#[derive(Debug)]
pub enum GptError {
    ApiError(String),
    JsonError(serde_json::Error),
}

impl fmt::Display for GptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GptError::ApiError(e) => write!(f, "ApiError: {}", e),
            GptError::JsonError(e) => write!(f, "JsonError: {}", e),
        }
    }
}

impl Error for GptError {}

impl From<serde_json::Error> for GptError {
    fn from(err: serde_json::Error) -> GptError {
        GptError::JsonError(err)
    }
}
