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

#### Step 3: Separation of Concerns for Binary Projects

Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

1. Split your program into a `main.rs` and a `lib.rs` and move your program’s logic to `lib.rs`.
2. As long as your command line parsing logic is small, it can remain in `main.rs`.
3. When the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`.

The responsibilities that remain in the main function after this process should be limited to the following:

1. Calling the command line parsing logic with the argument values
2. Setting up any other configuration
3. Calling a run function in `lib.rs`
4. Handling the error if run returns an error

#### Step 4: Test driven development

We’ll add the searching logic to the minigrep program by using the `Test-driven development (TDD)` process. This software development technique follows these steps:

1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!

We’ll test drive the implementation of the functionality that will actually do the searching for the query string in the file contents and produce a list of lines that match the query. We’ll add this functionality in a function called `search`.
