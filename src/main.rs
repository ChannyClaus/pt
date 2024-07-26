mod normalizer;
use ruff_python_parser;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let binding = fs::read_to_string(path).unwrap();
    let source = binding.as_str();

    println!("test_file: {}", path);
    let parsed = ruff_python_parser::parse(&source, ruff_python_parser::Mode::Module).unwrap();
    println!("parsed: {:?}", parsed);
    normalizer::Normalizer.visit_module(&mut parsed.into_syntax());
    println!("display: ")
    // let comment_ranges = ruff_python_trivia::CommentRanges::new(vec![]);
    // let formatted = ruff_python_formatter::format_module_ast(
    //     &parsed,
    //     &comment_ranges,
    //     source,
    //     ruff_python_formatter::PyFormatOptions::default(),
    // );
    // println!(
    //     "formatted.print(): {}",
    //     formatted.unwrap().print().unwrap().as_code()
    // );
}
