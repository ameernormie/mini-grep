use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents =
        fs::read_to_string("poem.txt").expect("Something went wrong while reading the file");

    println!("contents of poem {}", contents);

    println!("query is {}", config.query);
    println!("filename is {}", config.filename);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}
