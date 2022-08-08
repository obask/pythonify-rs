mod visitor;
mod mlir;

use rustpython_parser::ast::Suite;
use rustpython_parser::parser::parse_program;
use crate::visitor::PyAstVisitor;
// use crate::visitor;

fn main() {
    let contents = "\
    def func1(x: int) -> int:
        return x + 1
    ";
    let tmp: Suite = parse_program(contents).unwrap();
    let v = PyAstVisitor::new();
    let x = v.visit_suite(tmp);
    println!("{:?}", x);
}
