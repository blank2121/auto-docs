pub mod chat;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    match chat::gpt_request_user("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", "say hello world").await {
        Some(x) => {
            let data = x;
        },
        None => {}
    };
    Ok(())
}
