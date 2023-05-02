// with the help of Owen, I have named this code Dixy :) 
// the moody 14 year old girl that is way to edgy.

pub mod chat;
use tokio;
use std::fs::{File, read_dir};
use std::io::{BufReader, Read};
use std::env::{args, current_dir, set_current_dir};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

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

    set_current_dir(Path::new(&dir))?;

    let (system_config, api_key): (chat::Config, String) = chat::Config::load_file(
        &current_dir()?
        .into_os_string()
        .into_string()
        .unwrap()
        ).expect("config load error");




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

    for f in files_to_read {
        let file = File::open(f).expect("failed to open file");
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        let message = chat::Message {
            role: chat::Role::user,
            content: format!("make documentation for the following:\n{}", contents),
        };

        let request = chat::GptRequest::new(vec![system_config.as_ref().unwrap().clone(), 
                                            message], 1.0);

        println!("{:?}", request.send_request(&api_key, true).await?[0]);
    }

    Ok(())
}
