// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

use ruff_python_parser;
use std::{env, fs};

// This is the main function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let test_root = &args[1];
    println!("Test root: {}", test_root);
    let paths = fs::read_dir(test_root).unwrap();

    for path in paths {
        let parsed = ruff_python_parser::parse(
            path.unwrap().path().to_str().unwrap(),
            ruff_python_parser::Mode::Module,
        );
        println!("{:#?}", parsed);
    }
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}
