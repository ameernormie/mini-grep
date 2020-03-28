use std::error::Error;
use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    // using () like this is the idiomatic way to indicate that we’re calling run
    // for its side effects only; it doesn’t return a value we need.
    Ok(())
}
