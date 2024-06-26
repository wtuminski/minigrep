use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough argumenrts");
        };

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case_from_args = args.get(3);
        let ignore_case = match ignore_case_from_args {
            Some(value) => value.contains("IGNORE_CASE"),
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let lowercased_query = query.to_lowercase();
    contents
        .lines()
        .into_iter()
        .fold(vec![], |mut result, line| {
            if ignore_case && line.to_lowercase().contains(&lowercased_query) {
                result.push(line.trim())
            } else if !ignore_case && line.contains(query) {
                result.push(line.trim())
            }
            result
        })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const CONTENTS: &str = "\
         Rust:
         safe, fast, productive.
         Pick three.
         Rust rocks.
         Duct tape.";

    #[test]
    fn zero_results() {
        // given
        let query = "not_exisiting_part";
        let expected_result: Vec<&str> = vec![];

        // when
        let result = search(&query, &CONTENTS, false);

        // then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn case_sensitive_one_result() {
        // given
        let query = "duct";
        let expected_result = vec!["safe, fast, productive."];

        // when
        let result = search(&query, &CONTENTS, false);

        // then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn case_insensitive_two_results() {
        // given
        let query = "rUsT";
        let expected_result = vec!["Rust:", "Rust rocks."];

        // when
        let result = search(&query, &CONTENTS, true);

        // then
        assert_eq!(result, expected_result);
    }
}
