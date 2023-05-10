// with the help of Owen, I have named this code Dixy :) 
// the moody 14 year old girl that is way to edgy.

pub mod chat;
use tokio;
use std::fs::{File, read_dir, create_dir, metadata, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::env::{args, current_dir, set_current_dir};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut dir = current_dir()?.into_os_string().into_string().unwrap();

    let cli_args: Vec<String> = args().skip(1).collect();

    if cli_args.len() > 0 {
        if let Some(path) = cli_args.get(0) {
            dir = path.to_string();
        }
    } 

    set_current_dir(Path::new(&dir)).expect("Could not go to specified DIR");

    let (system_config, api_key): (chat::Config, String) = chat::Config::load_file(
        &current_dir()?
        .into_os_string()
        .into_string()
        .unwrap()
        ).expect("Could not find auto-doc.yaml file");




    let ignores = &system_config.ignore_files;
    let ignores = ignores.iter().map(|x| PathBuf::from(x)).collect::<Vec<PathBuf>>();
    let system_config = &system_config.config_to_system_prompt();
    let file_in_cwd = get_files_in_dirs(vec![current_dir()?.into_os_string().into_string().unwrap().into()]);
    let ignores = get_files_in_dirs(ignores);
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
