// with the help of Owen, I have named this code Dixy :) 
// the moody 14 year old girl that is way to edgy.

pub mod chat;
use tokio;
use structopt::StructOpt;
use std::fs::{File, read_dir, create_dir, metadata, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::env::{current_dir, set_current_dir};
use std::path::PathBuf;
use std::collections::HashSet;
use std::panic;

#[derive(StructOpt, Debug)]
#[structopt(name = "auto-docs")]
struct Opt {
    /// Generate base auto_docs.yaml file
    #[structopt(short = "c", long = "config")]
    use_config: bool,

    /// Changes working directory for auto-docs
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: Option<PathBuf>,
}


fn folder_exists(folder_name: &str) -> bool {
    match metadata(folder_name) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

fn get_files_in_dirs(dirs: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for dir in dirs {
        if dir.is_dir() {
            for entry in read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_file() {
                    files.push(path.clone());
                } else if path.is_dir() {
                    files.append(&mut get_files_in_dirs(vec![path]));
                }
            }
        } else {
            files.push(dir);
        }
    }

    files
}

fn remove_duplicates(vec1: Vec<String>, vec2: Vec<String>) -> Vec<String> {
    let set1: HashSet<String> = vec1.clone().into_iter().collect();
    let set2: HashSet<String> = vec2.clone().into_iter().collect();
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    let mut result: Vec<String> = vec1.into_iter().chain(vec2.into_iter()).collect();
    result.retain(|x| !intersection.contains(x));
    result
}

fn create_auto_docs_file() -> std::io::Result<()> {
    let mut file = File::create("auto_docs.yaml")?;

    let contents = "# Configuration file for My Documentation Project
# powered by chatgpt-3.5-turbo so promts are like asking a chat.openai.com
# the prompt will be exactly what is in the system prompt so be specific

system_prompt: \"here is my definition of documentation and how it should be made if you are asked
  to make documentation. Firstly, you will only respond with the documentation and this is how you
  will produce the documentation. In markdown, make a h2 title for the function name and underneath
  write only the initialization line that definition of the function, underneath that write a brief
  description, finally write a code example (h4) for how to use the function and end that function with
  a line break.\"
lang_specific_information: \"none\"
ignore_files:
  - 

function_description_length: \"max length of a paragraph and a half. try to keep it concise\"
include_overall_summary: false

api_key: \"\"";

    file.write_all(contents.as_bytes())?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    
    if opt.path.is_some() {
        set_current_dir(opt.path.unwrap()).expect("Could not change DIR");
    }
    if opt.use_config {
        create_auto_docs_file().expect("Could not create config file");
        println!("\nCreated base auto_docs.yaml file!");
        return Ok(());
    }

    let (system_config, api_key): (chat::Config, String) = match chat::Config::load_file(
        &current_dir()?
        .into_os_string()
        .into_string()
        .unwrap()
        ) {
        Ok(x) => {x},
        Err(_) => {
            panic::set_hook(Box::new(|_| {
                println!("Could not find auto_docs.yaml or not all config parmeters were filled out");
            }));

            panic!("test");
        },
    };


    let mut ignores = system_config.ignore_files.clone();
    if !ignores.contains(&"auto_docs.yaml".to_string()) {
        ignores.push("auto_docs.yaml".to_string());
    }
    if !ignores.contains(&"generated_docs".to_string()) {
        ignores.push("generated_docs".to_string());
    }
    let ignores = ignores.iter().map(|x| PathBuf::from(x)).collect::<Vec<PathBuf>>();
    let system_config = &system_config.config_to_system_prompt();
    let file_in_cwd = get_files_in_dirs(vec![current_dir()?.into_os_string().into_string().unwrap().into()]);
    let mut ignores = get_files_in_dirs(ignores);
    ignores.retain(|x| x.exists());
    let ignores = ignores.iter().map(|x| x.canonicalize().unwrap()).collect::<Vec<PathBuf>>();
    let files_to_read = remove_duplicates(ignores
                                          .iter()
                                          .map(|x| x.clone().into_os_string()
                                          .into_string()
                                          .unwrap())
                                          .collect::<Vec<String>>(),
                                          file_in_cwd.iter()
                                          .map(|x| x.clone().into_os_string()
                                          .into_string()
                                          .unwrap())
                                          .collect::<Vec<String>>()); 
    

    println!("Processing code...\n\n");

    let folder_name = "generated_docs";
    if !folder_exists(folder_name) {
        match create_dir(folder_name) {
            Ok(_) => println!("\nDocs folder created!\n"),
            Err(e) => println!("\nError creating folder: {}\n", e),
        }
    }

    for f in files_to_read {
        let file = File::open(&f).expect("Failed to open file");
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).expect("Could not read file");

        let message = chat::Message {
            role: chat::Role::user,
            content: format!("Make documentation for the following:\n{}", contents),
        };

        let request = chat::GptRequest::new(vec![system_config.as_ref().unwrap().clone(), 
                                            message], 0.5);

        let res: &str = &request.send_request(&api_key, true).await?[0];
        let mut res: String = res.replace("\\n", "\n");
        res.remove(0);
        res.pop();
       
        let file_name = format!("{}", PathBuf::from(&f)
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .to_string());
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(format!("./generated_docs/{}.md", &file_name))
            .expect("Error creating file");

        match file.write_all(&res.as_bytes()) {
            Ok(_) => println!("Generated {}\n", file_name),
            Err(e) => println!("Error writing to file: {}", e),
        }   
    }

    println!("Finished!!!");
    Ok(())
}
