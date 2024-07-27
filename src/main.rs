mod normalizer;
use ruff_python_ast::{str::Quote, Mod, ModModule};
use ruff_python_codegen::{stylist::Indentation, Generator};
use ruff_python_parser::{self};
use ruff_source_file::LineEnding;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let binding = fs::read_to_string(path).unwrap();
    let source = binding.as_str();

    println!("reading {}...", path);
    let parsed = ruff_python_parser::parse(&source, ruff_python_parser::Mode::Module).unwrap();
    let mut syntax = parsed.into_syntax();
    normalizer::Normalizer.visit_module(&mut syntax);

    let indentation = Indentation::default();
    let quote = Quote::default();
    let line_ending = LineEnding::default();
    let mut generator = Generator::new(&indentation, quote, line_ending);

    generator.unparse_suite(&syntax.as_module().unwrap().body);
    println!("{}", generator.generate());
}
