use crate::ruff_python_ast::Stmt::ClassDef;
use crate::ruff_python_ast::Stmt::FunctionDef;
use ruff_formatter::SourceCode;
use ruff_python_ast::{self};
use ruff_python_formatter::AsFormat;
use ruff_python_parser;
use ruff_source_file::Locator;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let binding = fs::read_to_string(path).unwrap();
    let source = binding.as_str();

    println!("test_file: {}", path);
    let parsed = ruff_python_parser::parse(&source, ruff_python_parser::Mode::Module).unwrap();
    // println!("parsed: {:#?}", parsed);
    match parsed.syntax() {
        ruff_python_ast::Mod::Module(module) => {
            // module.body = module.body[..1].to_vec();
            let source_code = SourceCode::new(source);
            //let comments = Comments::from_ast(parsed.syntax(), source_code, comment_ranges);
            let locator = Locator::new(source);

            // let formatted = format!(
            //   PyFormatContext::new(options, locator.contents(), comments, parsed.tokens()), [module.format()],
            // )
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

    let comment_ranges = ruff_python_trivia::CommentRanges::new(vec![]);
    let formatted = ruff_python_formatter::format_module_ast(
        &parsed,
        &comment_ranges,
        source,
        ruff_python_formatter::PyFormatOptions::default(),
    );
    println!(
        "formatted.print(): {}",
        formatted.unwrap().print().unwrap().as_code()
    );
}
