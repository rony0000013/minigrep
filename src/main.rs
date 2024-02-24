use minigrep::{run, Config};
use std::env;
use std::process;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err.red());
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("{}", format!("Application error: {e}").red());
        process::exit(1);
    }
}