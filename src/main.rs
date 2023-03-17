use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}