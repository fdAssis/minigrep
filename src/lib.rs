use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config<'a> {
    query: &'a String,
    filename: &'a String,
    case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let outcome = match config.case_sensitive {
        true => search(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents),
    };

    for line in outcome {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut outcome: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            outcome.push(line);
        }
    }

    outcome
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut outcome = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            outcome.push(line);
        }
    }

    outcome
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}
