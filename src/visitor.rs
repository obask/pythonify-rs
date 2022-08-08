use std::mem::swap;
use rustpython_parser::ast::{Stmt, StmtKind, Suite};
use crate::mlir::{Block, Label, Operator, Region};

pub(crate) struct PyAstVisitor {
    x: f64,
}

impl PyAstVisitor {
    pub fn new() -> Self {
        return Self { x: 0.0 };
    }

    pub fn visit_suite(&self, suite: Suite) -> Operator {
        let mut items: Vec<Operator> = vec![];
        for stmt in suite {
            items.push(self.visit_stmt(stmt));
        }
        let label = Label { name: "^bb0".to_string() };
        let vec1 = vec![vec![Block { label, items }]];
        return Operator { name: "module".to_string(), regions: vec1 };
    }

    fn visit_stmt(&self, stmt: Stmt) -> Operator {
        let mut op = Operator{ name: "111".to_string(), regions: vec![] };
        let kind = stmt.node;
        match kind {
            StmtKind::FunctionDef { .. } => {
                println!("{:?}", kind);
            }
            StmtKind::AsyncFunctionDef { .. } => {
                println!("{:?}", kind);
            }
            StmtKind::ClassDef { .. } => {}
            StmtKind::Return { .. } => {}
            StmtKind::Delete { .. } => {}
            StmtKind::Assign { .. } => {}
            StmtKind::AugAssign { .. } => {}
            StmtKind::AnnAssign { .. } => {}
            StmtKind::For { .. } => {}
            StmtKind::AsyncFor { .. } => {}
            StmtKind::While { .. } => {}
            StmtKind::If { .. } => {}
            StmtKind::With { .. } => {}
            StmtKind::AsyncWith { .. } => {}
            StmtKind::Raise { .. } => {}
            StmtKind::Try { .. } => {}
            StmtKind::Assert { .. } => {}
            StmtKind::Import { .. } => {}
            StmtKind::ImportFrom { .. } => {}
            StmtKind::Global { .. } => {}
            StmtKind::Nonlocal { .. } => {}
            StmtKind::Expr { .. } => {}
            StmtKind::Pass => {}
            StmtKind::Break => {}
            StmtKind::Continue => {}
        }
        return op;
    }
}

