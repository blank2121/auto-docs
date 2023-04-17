pub mod chat;
use tokio;
use std::fs::File;
use std::io::{BufReader, Read};

// refactor chat code to make messages and then send chained versions of them
// also get back raw or parsed
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let file = File::open("./test_docs/src/lib.rs")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let config = chat::Config::load_file("./test_docs");
    let config = config?.config_to_system_prompt();

    let message = chat::Message {
        role: chat::Role::user,
        content: format!("make documentation for the following:\n{}", contents),
    };
    

    let request = chat::GptRequest::new(vec![config?, message], 1.0);

    println!("{:?}", request.send_request("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", true).await?[0]);

    Ok(())
}
