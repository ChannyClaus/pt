use crate::ruff_python_ast::Stmt::ClassDef;
use crate::ruff_python_ast::Stmt::FunctionDef;
use ruff_python_ast::{self};
use ruff_python_parser;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let source = fs::read_to_string(path).unwrap();

    println!("test_file: {}", path);
    let mut parsed = ruff_python_parser::parse(&source, ruff_python_parser::Mode::Module).unwrap();
    println!("parsed: {:#?}", parsed);
    match parsed.syntax {
        ruff_python_ast::Mod::Module(ref mut module) => {
            module.body = module.body[..1].to_vec();
            for stmt in &module.body {
                match stmt {
                    FunctionDef(stmt) => {
                        println!("stmt: {:#?}", stmt.name.id.to_string());
                    }
                    ClassDef(stmt) => {
                        println!("stmt: {:#?}", stmt.name.id.to_string());
                    }
                    _ => continue,
                }
            }
        }
        _ => {
            println!("not module");
        }
    }

    let comment_ranges = ruff_python_trivia::CommentRanges::from(parsed.tokens());
    let formatted = ruff_python_formatter::format_module_ast(
        &parsed,
        &comment_ranges,
        source.as_str(),
        ruff_python_formatter::PyFormatOptions::default(),
    );
    println!(
        "formatted.print(): {}",
        formatted.unwrap().print().unwrap().as_code()
    );
}
