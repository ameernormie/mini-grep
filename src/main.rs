use std::env;

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

    println!("query is {}", query);
    println!("filename is {}", filename);
}
