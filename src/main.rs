use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut query = "";
    let mut filename = "";

    if args.len() == 2 {
        query = &args[1];
    }
    if args.len() > 2 {
        query = &args[1];
        filename = &args[2];
    }

    let contents =
        fs::read_to_string("poem.txt").expect("Something went wrong while reading the file");

    println!("contents of poem {}", contents);

    println!("query is {}", query);
    println!("filename is {}", filename);
}
