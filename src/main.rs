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
pub struct Test {
    pub name: String,
    pub path: String,
}

impl Test {
    pub fn run(self, py: Python) {
        let import_path = self.path.replace(".py", "").replace("/", ".");
        let imported = py.import_bound(import_path.as_str()).unwrap();
        let result = imported.getattr(self.name.as_str()).unwrap().call0();
        match result {
            Ok(_) => info!("{} passed", self.name),
            Err(e) => error!("{} failed: {:#?}", self.name, e),
        }

        // let main = py.import_bound(self.path).unwrap();
    }
}

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
    pub fn run(self, py: Python) {
        debug!("running package: {:#?}", self.source);
        for module in self.modules.into_iter() {
            module.run(py)
        }
    }
    pub fn from_dir(path: &str) -> Self {
        let paths = fs::read_dir(path).unwrap();

        let mut subpackages = vec![];
        let mut modules = vec![];

        for path in paths {
            let p = path.unwrap().path();
            let filename = p.file_name().unwrap().to_str().unwrap();
            if p.is_dir() && !filename.contains("/.") {
                subpackages.push(Package::from_dir(p.to_str().unwrap()));
                continue;
            }

            if filename.starts_with("test") && filename.ends_with(".py") {
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
    pub tests: Vec<Test>,
}

impl Module {
    pub fn run(self, py: Python) {
        debug!("running module: {:#?}", self.path);
        for test in self.tests.into_iter() {
            test.run(py);
        }
    }
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
        if metadata(&args[1].clone()).unwrap().is_file() {
            let module = Module::from_file(&args[1].clone());
            module.run(py);
            return;
        }
        let package = Package::from_dir(&args[1].clone());
        package.run(py);
    });
}
