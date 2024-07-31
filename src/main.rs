use pyo3::prelude::*;
mod normalizer;
use ruff_python_ast::{str::Quote, StmtFunctionDef};
use ruff_python_codegen::{stylist::Indentation, Generator};
use ruff_python_parser::{self};
use ruff_source_file::LineEnding;
use std::{env, fs};

#[derive(Debug)]
pub struct TestFile {
    pub name: String,
    pub fixtures: Vec<String>,
    pub tests: Vec<String>,
}

fn parse_testfile(path: &str) -> TestFile {
    let source = fs::read_to_string(path).unwrap();
    let parsed = ruff_python_parser::parse(&source, ruff_python_parser::Mode::Module).unwrap();
    let syntax = parsed.into_syntax();

    let mut tests = vec![];
    let mut fixtures = vec![];
    for stmt in syntax.as_module().unwrap().body.iter() {
        match stmt {
            ruff_python_ast::Stmt::FunctionDef(StmtFunctionDef {
                name,
                decorator_list,
                ..
            }) => {
                if name.starts_with("test") {
                    tests.push(name.to_string());
                } else {
                    for decorator in decorator_list.iter() {
                        match &decorator.expression {
                            ruff_python_ast::Expr::Attribute(attr) => match *attr.value.clone() {
                                ruff_python_ast::Expr::Name(expr_name) => {
                                    if expr_name.id.to_string() == "ptst"
                                        && attr.attr.id.to_string() == "fixture"
                                    {
                                        fixtures.push(name.to_string());
                                    }
                                }
                                _ => {
                                    println!("asdf");
                                }
                            },
                            _ => {
                                println!("asdf");
                            }
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    TestFile {
        name: path.to_string(),
        fixtures,
        tests,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let paths = get_paths(args[1].clone()).unwrap();
    let paths = fs::read_dir(args[1].clone())
        .unwrap()
        .into_iter()
        .map(|entry| parse_testfile(entry.unwrap().path().to_str().unwrap()))
        .collect::<Vec<_>>();
    println!("{:#?}", paths);

    // TODO:
    // 0. validate the existing test files (fixtures being required all exist, etc)
    // 1. copy the test dir to a temp dir (via copy-on-write)
    // 2. transform the test files via the AST
    // 3. run the tests via pyo3.

    // let path = &args[1];
    // let binding = fs::read_to_string(path).unwrap();
    // let source = binding.as_str();

    // let parsed = ruff_python_parser::parse(&source, ruff_python_parser::Mode::Module).unwrap();
    // let mut syntax = parsed.into_syntax();
    // normalizer::Normalizer.visit_module(&mut syntax);

    // let indentation = Indentation::default();
    // let quote = Quote::default();
    // let line_ending = LineEnding::default();
    // let mut generator = Generator::new(&indentation, quote, line_ending);

    // generator.unparse_suite(&syntax.as_module().unwrap().body);
    // fs::write("generated.py", generator.generate()).unwrap();

    // for test_name in test_names {
    //     Python::with_gil(|py| {
    //         let main = py.import_bound("generated").unwrap();
    //         let test: Py<PyAny> = main.getattr(test_name.to_string().as_str()).unwrap().into();
    //         let result = test.call0(py);
    //         match result {
    //             Ok(_) => println!("{} passed", test_name),
    //             Err(e) => println!("{} failed: {:#?}", test_name, e),
    //         }
    //     })
    // }
}
