use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough argumenrts");
        };

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .into_iter()
        .fold(vec![], |mut result, line| {
            if line.contains(query) {
                result.push(line.trim())
            }
            result
        })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
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
         Rust rocks.";

    #[test]
    fn zero_results() {
        // given
        let query = "not_exisiting_part";
        let expected_result: Vec<&str> = vec![];

        // when
        let result = search(&query, &CONTENTS);

        // then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn one_result() {
        // given
        let query = "duct";
        let expected_result = vec!["safe, fast, productive."];

        // when
        let result = search(&query, &CONTENTS);

        // then
        assert_eq!(result, expected_result);
    }

    #[test]
    fn two_result() {
        // given
        let query = "Rust";
        let expected_result = vec!["Rust:", "Rust rocks."];

        // when
        let result = search(&query, &CONTENTS);

        // then
        assert_eq!(result, expected_result);
    }
}
