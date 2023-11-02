use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let mut ignore_case = false;

        let is_env_ignore_case_set = env::var("IGNORE_CASE").is_ok();

        if is_env_ignore_case_set {
            let value = env::var("IGNORE_CASE").unwrap_or_else(|_| "false".to_string());
            let value: bool = match value.parse() {
                Ok(val) => val,
                Err(_) => return Err("IGNORE_CASE must be a boolean"),
            };

            ignore_case = value
        }

        if args.len() > 3 {
            let is_argument_ignore_case: bool = match args[3].clone().parse() {
                Ok(value) => value,
                Err(_) => {
                    return Err("third argument must be a boolean value");
                }
            };

            if !ignore_case && is_argument_ignore_case {
                ignore_case = true
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for lines in contents.lines() {
        if lines.contains(query) {
            result.push(lines);
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for lines in contents.lines() {
        if lines.to_lowercase().contains(&query.to_lowercase()) {
            result.push(lines);
        }
    }

    result
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
