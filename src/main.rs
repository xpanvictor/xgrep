use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {} in {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading file");

    println!("With text:\n{}", contents)
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
