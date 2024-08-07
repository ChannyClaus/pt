use pyo3::prelude::*;
mod normalizer;
use ruff_python_ast::{Expr, ExprAttribute, ExprCall, ExprName, Stmt, StmtFunctionDef};
use ruff_python_parser::{self, parse_module};
use std::{
    env,
    fs::{self, metadata},
};
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct Module {
    pub path: String,
    pub fixtures: Vec<Fixture>,
    pub tests: Vec<Test>,
}

#[derive(Debug)]
pub struct Test {
    pub path: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Fixture {
    pub name: String,
    pub path: String,
    pub scope: String,
    pub autouse: bool,
}

impl Module {
    pub fn from_file(path: &str) -> Self {
        let source = fs::read_to_string(path).unwrap();
        let parsed = parse_module(source.as_str()).unwrap();

        let mut tests = vec![];
        let mut fixtures = vec![];
        for stmt in parsed.suite() {
            if let Stmt::FunctionDef(StmtFunctionDef {
                name,
                decorator_list,
                ..
            }) = stmt
            {
                if name.starts_with("test") {
                    tests.push(Test {
                        name: name.to_string(),
                        path: path.to_string(),
                    });
                    continue;
                }
                for decorator in decorator_list.iter() {
                    match &decorator.expression {
                        Expr::Call(ExprCall {
                            func, arguments, ..
                        }) => {
                            let attr_expr = func.as_attribute_expr().unwrap();
                            // println!(
                            //     "{:#?}",
                            //     arguments.keywords[0].arg.clone().unwrap().to_string()
                            // );
                            // let Expr::BooleanLiteral(ExprBooleanLiteral { value, .. }) =
                            //     arguments.keywords[0].value.clone()
                            // else {
                            //     todo!()
                            // };

                            match *attr_expr.value.clone() {
                                Expr::Name(fixture_name) => {
                                    if fixture_name.id.to_string() == "ptst".to_string()
                                        && attr_expr.attr.to_string() == "fixture".to_string()
                                    {
                                        fixtures.push(Fixture {
                                            name: name.id.to_string(),
                                            scope: "function".to_string(),
                                            autouse: false,
                                            path: path.to_string(),
                                        });
                                    }
                                }
                                _ => todo!(),
                            }
                        }
                        Expr::Attribute(ExprAttribute { value, attr, .. }) => {
                            match (**value).clone() {
                                Expr::Name(ExprName { id, .. }) => {
                                    if id.to_string() == "ptst".to_string()
                                        && attr.to_string() == "fixture".to_string()
                                    {
                                        fixtures.push(Fixture {
                                            name: name.id.to_string(),
                                            scope: "function".to_string(),
                                            autouse: false,
                                            path: path.to_string(),
                                        });
                                    }
                                }
                                _ => todo!(),
                            }
                        }
                        _ => todo!(),
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
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    info!("starting the session");
    Python::with_gil(|py| {
        let args: Vec<String> = env::args().collect();
        let path = args[1].clone();
        let module = Module::from_file(&path);
        println!("module: {:#?}", module);
        let bound =
            PyModule::from_code_bound(py, fs::read_to_string(path).unwrap().as_str(), "", "")
                .unwrap();

        for test in module.tests.iter() {
            let result = bound.getattr(test.name.as_str()).unwrap().call0();
            match result {
                Ok(_) => info!("{} passed", &test.name),
                Err(e) => error!("{} failed: {:#?}", &test.name, e),
            }
        }
        return;
    });
}
