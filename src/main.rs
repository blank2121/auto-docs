pub mod chat;
use tokio;
use std::fs::{File, read_dir, metadata};
use std::io::{BufReader, Read};
use std::env::{args, current_dir, set_current_dir};
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut dir = current_dir()?.into_os_string().into_string().unwrap();

    let cli_args: Vec<String> = args().skip(1).collect();

    if cli_args.len() > 0 {
        if let Some(path) = cli_args.get(0) {
            dir = path.to_string();
        }
    } 


    set_current_dir(Path::new(&dir))?;

    let (config, api_key) = chat::Config::load_file(
        &current_dir()?
        .into_os_string()
        .into_string()
        .unwrap()
        ).expect("config load error");

    let config = config.config_to_system_prompt();


    let file = File::open("./src/lib.rs").expect("failed to open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    println!("read file");

    for entry in read_dir(current_dir()?).expect("could not read dir for loop") {
        let entry = entry?;
        let path = entry.path();

        let metadata = metadata(&path)?;
        println!(
                "Is Dir: {:?}, is file: {:?}, filename: {:?}",
                path.is_dir(),
                path.is_file(),
                path.file_name().ok_or("No filename")?
            );

    }


    

    let message = chat::Message {
        role: chat::Role::user,
        content: format!("make documentation for the following:\n{}", contents),
    };
    
    todo!("halt");

    let request = chat::GptRequest::new(vec![config?, message], 1.0);

    println!("{:?}", request.send_request(&api_key, true).await?[0]);

    Ok(())
}
