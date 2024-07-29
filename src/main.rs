use pyo3::prelude::*;
mod normalizer;
use ruff_python_ast::{str::Quote, StmtFunctionDef};
use ruff_python_codegen::{stylist::Indentation, Generator};
use ruff_python_parser::{self};
use ruff_source_file::LineEnding;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let binding = fs::read_to_string(path).unwrap();
    let source = binding.as_str();

    // println!("reading {}...", path);
    let parsed = ruff_python_parser::parse(&source, ruff_python_parser::Mode::Module).unwrap();
    let mut syntax = parsed.into_syntax();
    normalizer::Normalizer.visit_module(&mut syntax);

    let indentation = Indentation::default();
    let quote = Quote::default();
    let line_ending = LineEnding::default();
    let mut generator = Generator::new(&indentation, quote, line_ending);

    generator.unparse_suite(&syntax.as_module().unwrap().body);
    fs::write("generated.py", generator.generate()).unwrap();

    let mut test_names = vec![];
    for stmt in syntax.as_module().unwrap().body.iter() {
        match stmt {
            ruff_python_ast::Stmt::FunctionDef(StmtFunctionDef { name, .. }) => {
                if name.starts_with("test") {
                    test_names.push(name.clone());
                }
            }
            _ => continue,
        }
    }

    for test_name in test_names {
        Python::with_gil(|py| {
            let main = py.import_bound("generated").unwrap();
            let test: Py<PyAny> = main.getattr(test_name.to_string().as_str()).unwrap().into();
            let result = test.call0(py);
            match result {
                Ok(_) => println!("{} passed", test_name),
                Err(e) => println!("{} failed: {:#?}", test_name, e),
            }
        })
    }
}
