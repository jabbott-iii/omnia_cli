pub mod ls;
pub mod art;

use owo_colors::OwoColorize;
/*use clap::Parser;
use std::path::PathBuf;

// Command-line interface definition
#[derive(Debug, Parser)]
#[command(version, author = "Joseph Abbott III", about = "An all in one CLI tool.")]
pub struct Cli {
    path: Option<PathBuf>,
} */

pub fn core_run() {
    
    println!("{:?}", art::home_page());

    loop {

        // Display prompt and read user input
        println!("{}", "Omnia CLI - Enter a command (type 'exit' to quit):".bright_red());
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();

        if command.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting Omnia CLI. Goodbye!".bright_red());
            break;
        }
        match command {
            "ls" => ls::ls_complete(),
            _ => println!("{}", "Unknown command. Please try again.".red()),
        }
    }
}