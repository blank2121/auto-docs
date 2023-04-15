pub mod chat;
use tokio;
use std::env::current_dir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = match chat::gpt_request_user("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", "say hello world").await {
        Some(x) => x,
        None => vec!["oops!".to_string()]
    };

    println!("{:?}", data);
    
    let x = chat::Config::load_file("./test_docs"); 
    println!("{:?}", current_dir()?);
    dbg!(x?);
    Ok(())
}
