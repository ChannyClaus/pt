use pyo3::prelude::*;
mod normalizer;
use ruff_python_ast::{str::Quote, Expr, ExprAttribute, Stmt, StmtFunctionDef};
use ruff_python_codegen::{stylist::Indentation, Generator};
use ruff_python_parser::{self, parse};
use ruff_source_file::LineEnding;
use std::{
    env,
    fs::{self, metadata},
};

#[derive(Debug)]
pub struct Fixture {
    pub name: String,
}

#[derive(Debug)]
pub struct Package {
    pub source: String,
    pub modules: Vec<Module>,
    pub subpackages: Vec<Package>,
}

impl Package {
    pub fn from_dir(path: &str) -> Self {
        let paths = fs::read_dir(path).unwrap();

        let mut subpackages = vec![];
        let mut modules = vec![];

        for path in paths {
            let p = path.unwrap().path();
            let pstr = p.to_str().unwrap().to_string();
            if p.is_dir() && !pstr.contains("/.") {
                subpackages.push(Package::from_dir(p.to_str().unwrap()));
                continue;
            }

            if pstr.starts_with("test") && pstr.ends_with(".py") {
                modules.push(Module::from_file(p.to_str().unwrap()));
                continue;
            }
        }

        Self {
            source: path.to_string(),
            modules,
            subpackages,
        }
    }
}

#[derive(Debug)]
pub struct Module {
    pub path: String,
    pub fixtures: Vec<Fixture>,
    pub tests: Vec<String>,
}

impl Module {
    pub fn from_file(path: &str) -> Self {
        let source = fs::read_to_string(path).unwrap();
        let parsed = parse(&source, ruff_python_parser::Mode::Module).unwrap();
        let syntax = parsed.into_syntax();

        let mut tests = vec![];
        let mut fixtures = vec![];
        for stmt in syntax.as_module().unwrap().body.iter() {
            if let Stmt::FunctionDef(StmtFunctionDef {
                name,
                decorator_list,
                ..
            }) = stmt
            {
                if name.starts_with("test") {
                    tests.push(name.to_string());
                    continue;
                }
                for decorator in decorator_list.iter() {
                    if let Expr::Attribute(ExprAttribute { value, attr, .. }) =
                        &decorator.expression
                    {
                        if let Expr::Name(expr_name) = &*value.clone() {
                            if expr_name.id.to_string() == "ptst"
                                && attr.id.to_string() == "fixture"
                            {
                                println!("decorator: {:#?}", decorator);
                                fixtures.push(Fixture {
                                    name: name.to_string(),
                                })
                            }
                        }
                    }
                }
            }
        }

        Self {
            path: path.to_string(),
            fixtures,
            tests,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if metadata(&args[1].clone()).unwrap().is_file() {
        let module = Module::from_file(&args[1].clone());
        // println!("module: {:#?}", module);
        return;
    }
    let package = Package::from_dir(&args[1].clone());
    // println!("package: {:#?}", package);
    // let paths = get_paths(args[1].clone()).unwrap();
    // let paths = fs::read_dir(args[1].clone())
    //     .unwrap()
    //     .into_iter()
    //     .map(|entry| Module::from_file(entry.unwrap().path().to_str().unwrap()))
    //     .collect::<Vec<_>>();
    // println!("{:#?}", paths);

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
