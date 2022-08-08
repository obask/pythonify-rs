mod visitor;
mod mlir;

use std::mem::swap;
use rustpython_parser::ast::Suite;
use rustpython_parser::parser;
use crate::mlir::Operator;
use crate::visitor::PyAstVisitor;

fn main() {
    let contents = "\
    def func1(x: int) -> int:
        return x + 1
    ";
    let tmp: Suite = parser::parse_program(contents).unwrap();
    let v = PyAstVisitor::new();
    let mut x = v.visit_suite(&tmp);
    let item: Operator = Operator { name: "".to_string(), regions: vec![] };
    let vv = &mut x.regions[0][0].items;
    vv.push(item);
    let tt = &mut x.regions[0][0].items[0];
    tt.name = "!!!!".to_string();
    println!("{:?}", x);
    // println!("{:?}", vectors);
}
