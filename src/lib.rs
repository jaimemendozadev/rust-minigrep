use std::error::Error;
use std::fs;
#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // Review &'static str on Page 204
        if args.len() < 3 {
            return Err("You are missing an argument");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn check_config_implementation() {
        let args = [String::from("blargh")];
        let first_config = Config::new(&args);

        assert_eq!(first_config, Err("You are missing an argument"));

        let second_args = [
            String::from("/some/random/path"),
            String::from("Are you"),
            String::from("Are you nobody, too?"),
        ];

        let second_config = Config::new(&second_args);

        assert_eq!(
            second_config,
            Ok(Config {
                query: String::from("Are you"),
                filename: String::from("Are you nobody, too?")
            })
        )
    }
}