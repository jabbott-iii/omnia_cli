use std::{env::current_dir, fs, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
#[command(version)]

struct CLI {
    path: Option<PathBuf>,
}

fn main() {
    let args = CLI::parse();
    let path = args.path.unwrap_or(PathBuf::from(current_dir().unwrap()));
    println!("Current path: {}", path.display());
    if let Ok(does_exist) = fs::exists(&path)  {
        if does_exist {
            for file in get_files(&path) {
                println!("{}", file);
            }
        } else {
            println!("{}", "The path does not exist.");
        }
    } 
    else {
            println!("{}", "Could not determine if the path exists.");
    }
}

fn get_files(path: &PathBuf) -> Vec<String> {
    let mut files = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                files.push(file.file_name()
                .into_string()
                .unwrap_or("unkown name".into()),
                );
            }
        }
    } else {
        files.push("Could not read directory.".into());
    } 
    files
}