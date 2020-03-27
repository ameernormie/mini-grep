# Building a Command Line Program

> Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language for creating command line tools, so for this project, we’ll make our own version of the classic command line tool `grep` (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a filename and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

### Concepts that will be covered

- Organizing code
- Using vectors and strings
- Handling errors
- Using traits and lifetimes where appropriate
- Writing tests

#### Step 1: Accepting Command Line Arguments

Read the command line arguments using the function provided by the Rust's standard library using `std::env::args`. This function returns an iterator of the command line arguments. We can call `collect` method on the iterator to turn it into a collection, such as a vector.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
}
```

#### Step 2: Reading a File

```rust
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("poem.txt").expect("Something went wrong while reading the file");

    println!("contents of poem {}", contents);
}
```
