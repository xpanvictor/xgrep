
//! This runs the entire grep functionalities.


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

/// Config struct with a constructor new() \
/// Use to generate a config
pub struct Config {
    /// this is what to search for
    pub query: String,
    /// this is the path to file we want to search
    pub filename: String,
    /// whether to make case case_insensitive or not
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
/// Search functionality is here \
/// This search ensures case_sensitive query \
/// @params: query => what to search for \
/// @params: content => contents of file to search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // using iterator to filter lines
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

}

/// Case_insensitive form of search
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(
            |line|
                line
                    .to_lowercase()
                    .contains(&query)
        )
        .collect()

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
