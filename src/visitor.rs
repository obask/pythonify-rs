use rustpython_parser::ast::{Stmt, StmtKind, Suite};
use crate::mlir::{Block, Label, Operator, Region};

pub(crate) struct PyAstVisitor {
    x: f64,
    y: f64,
}

impl PyAstVisitor {
    pub fn new() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }

    pub fn visit_suite(&self, s: Suite) -> Operator {
        for c in s {
            self.visit_stmt(c);
        }
        let b1 = Block {
            label: Label { name: "^bb0".to_string() },
            items: vec![],
        };
        let b2 = Block {
            label: Label { name: "^bb1".to_string() },
            items: vec![],
        };
        let r: Vec<Block> = vec![b1, b2];
        return Operator { name: "py.module".to_string(), regions: vec![r] };
    }

    fn visit_stmt(&self, s: Stmt) -> Operator {
        let mut op = Operator{ name: "".to_string(), regions: vec![] };
        match s.node {
            StmtKind::FunctionDef { name, .. } => {
                println!("{}", name);
            }
            StmtKind::AsyncFunctionDef { .. } => {}
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

