use std::{env, process};

use xgrepx::{Config, run};

fn main() {

    let config = Config::new(
        env::args()
    ).unwrap_or_else(|error| {
        eprintln!("Problems parsing arguments: \n{}", error);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

