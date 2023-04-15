use reqwest::Client;
use serde::Serialize;
use serde::Deserialize;
use serde_json; 
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use serde_yaml;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
   system_prompt: String,
   lang_specific_information: String,
   ignore_files: Vec<String>,
   include_function_examples: bool,
   function_description_length: String,
   ignore_private_functions: bool,
   include_overall_summary: bool,
}

impl Config {
    pub fn new(system_prompt: String, lang_specific_information: String, ignore_files: Vec<String>, include_function_examples: bool, function_description_length: String, ignore_private_functions: bool, include_overall_summary: bool) -> Self { Self { system_prompt, lang_specific_information, ignore_files, include_function_examples, function_description_length, ignore_private_functions, include_overall_summary } }

    pub fn load_file(directory: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = format!("{}/auto_doc.yaml", directory);
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;

        Ok(config)
    }
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

pub async fn gpt_request_system(api_key: &str, config: Option<Config>, content: &str) -> Option<Vec<String>> {
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let client = Client::new();

    if config.is_some() {
        content = format!("Your goal is to create documentation of code that is given
                          to you. the rest of this prompt is so you understand what to do specifically
                          and they will be in logic form, like use english: true.
                          {sys_prompt}. language specific information: {lang_info}. 
                          include example code for functions: {func_examples}.
                          function description length: {func_descr_len}.
                          ignore private functions: {private_func}.
                          make a summary for the code overall: {summary}",
                          sys_prompt=config?.system_prompt,
                          lang_info=config?.lang_specific_information,
                          func_examples=config?.include_function_examples,
                          func_descr_len=config?.function_description_length,
                          private_func=config?.ignore_private_functions,
                          summary=config?.include_overall_summary);

    }

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

    let mut responces: Vec<String> = vec![];

    for data in body["choices"].as_array().unwrap() {
        responces.push(data["message"]["content"].to_string());
    }
    Some(responces)
}

