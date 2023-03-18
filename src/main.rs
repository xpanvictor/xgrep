use std::{env, process};

use xgrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problems parsing arguments: \n{}", error);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

