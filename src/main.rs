use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problems parsing arguments: \n{}", error);
        process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguements; first is search query, second is filepath");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
