use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // err is in the closure. err argument appears in vertical pipes if result is Err
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    #[allow(unused_variables)]
    let contents =
        fs::read_to_string("poem.txt").expect("Something went wrong while reading the file");

    println!("query is {}", config.query);
    println!("filename is {}", config.filename);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
