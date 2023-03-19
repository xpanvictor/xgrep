use std::{fs, env, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        // skip into first argument
        args.next();

        // retrieve second value, query
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        // retrieve third value, filename
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case
        })
    }
}

// todo: use hashmap to add line number to result
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    // creating the returned vector of references
    let mut result_of_search: Vec<&'a str> = Vec::new();

    // iterating over lines
    for line in contents.lines() {
        // checking if line contains query
        if line.contains(query) {
            // pushing line as reference to the mutable results vector
            result_of_search.push(line);
        }
    }

    result_of_search
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results_of_search = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results_of_search.push(line);
        }
    }

    results_of_search
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
