use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Self { query, filename })
    }
}
