use ruff_python_parser;
use std::env;

// This is the main function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let test_file = &args[1];
    println!("test_file: {}", test_file);
    let parsed = ruff_python_parser::parse(&test_file, ruff_python_parser::Mode::Module);
    println!("{:#?}", parsed);
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}
