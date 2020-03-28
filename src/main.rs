use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

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

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
