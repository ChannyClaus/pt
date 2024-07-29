use itertools::Either::{Left, Right};

use ruff_python_ast::visitor::transformer;
use ruff_python_ast::visitor::transformer::Transformer;
use ruff_python_ast::{self as ast, Expr, Stmt};

pub struct Normalizer;

impl Normalizer {
    /// Transform an AST module into a normalized representation.
    #[allow(dead_code)]
    pub(crate) fn visit_module(&self, module: &mut ast::Mod) {
        match module {
            ast::Mod::Module(module) => {
                self.visit_body(&mut module.body);
            }
            ast::Mod::Expression(expression) => {
                self.visit_expr(&mut expression.body);
            }
        }
    }
}

impl Transformer for Normalizer {
    fn visit_stmt(&self, stmt: &mut Stmt) {
        match stmt {
            Stmt::FunctionDef(ast::StmtFunctionDef { range: _, .. }) => {
                // println!("inside function def");
                // println!("name: {}", name);
                // println!("decorator_list: {:#?}", decorator_list);
            }
            _ => {
                // println!("other things");
            }
        }

        if let Stmt::Delete(delete) = stmt {
            // Treat `del a, b` and `del (a, b)` equivalently.
            delete.targets = delete
                .targets
                .clone()
                .into_iter()
                .flat_map(|target| {
                    if let Expr::Tuple(tuple) = target {
                        Left(tuple.elts.into_iter())
                    } else {
                        Right(std::iter::once(target))
                    }
                })
                .collect();
        }

        transformer::walk_stmt(self, stmt);
    }
}
