use std::collections::HashMap;

use hime_redist::{
    ast::AstNode,
    symbols::{SemanticElementTrait, Symbol, SymbolType},
};

#[derive(Debug)]
pub struct Program {
    pub decl_f: Vec<Decl>,
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(root_node: AstNode) -> Self {
        let mut decl_f: Vec<Decl> = Vec::new();
        let mut stmts: Vec<Stmt> = Vec::new();

        for node in root_node.children() {
            match node.get_symbol().name {
                "declS" => {
                    for decl in node.children() {
                        //TODO
                    }
                }
                "stmtS" => {
                    for stmt in node.children() {
                        stmts.push(Stmt::new(stmt));
                    }
                }
                _ => panic!(),
            }
        }

        Program { decl_f, stmts }
    }
}

#[derive(Debug)]
pub enum Decl {
    Import {},
    Decl {},
}

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        name: String,
        declared_type: Type,
        value: Expr,
    },
}

impl Stmt {
    fn new(stmt: AstNode) -> Self {
        match stmt.get_symbol().name {
            "varDecl" => {
                //let identifier = stmt.children().iter().find(|child| child.get_symbol().name.eq("IDENTIFIER")).unwrap().get_value().unwrap().into();
                Stmt::VarDecl {
                    name: stmt.child(0).get_value().unwrap().into(),
                    declared_type: Type::new(stmt.child(1).get_value().unwrap()),
                    value: Expr::new(stmt.child(2)),
                }
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    BinaryOperation {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        operator: BinaryOperator
    }
}

impl Expr {
    fn new(expr: AstNode) -> Self {
        match expr.get_symbol().name {
            "INTEGER" => Expr::Integer(expr.get_value().unwrap().parse().unwrap()),
            "+" | "-" | "*" | "/" => {
                let lhs = Box::new(Expr::new(expr.child(0)));
                let rhs = Box::new(Expr::new(expr.child(1)));
                let operator = BinaryOperator::new(expr.get_symbol());

                Expr::BinaryOperation { lhs, rhs, operator }
            },
            "IDENTIFIER" => Expr::Variable(expr.get_value().unwrap().into()),
            "BOOLEAN" =>  Expr::Boolean(expr.get_value().unwrap().parse().unwrap()),
            _ => panic!("Expression type not found: {}", expr.get_symbol().name),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Expr::Integer(_) => Type::Int,
            Expr::Boolean(_) => Type::Bool,
            _ => panic!()
        }
    }

}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl BinaryOperator {
    fn new(operator: Symbol) -> Self {
        match operator.name {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => panic!()
        }
    }
}

//type TypeEnv = HashMap<String, Type>;
/*fn typecheck_expr(expr: &Expr, env: &TypeEnv) -> Result<Type, String> {
    match expr {
        Expr::Integer(_) => Ok(Type::Int),
        Expr::Variable(name) => {
            env.get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name))
        }
        Expr::BinaryOp { left, op: _, right } => {
            let l_type = typecheck_expr(left, env)?;
            let r_type = typecheck_expr(right, env)?;
            if l_type == Type::Int && r_type == Type::Int {
                Ok(Type::Int)
            } else {
                Err("Binary operations require integer operands".into())
            }
        }
    }
}*/
#[derive(Debug)]
pub enum Type {
    Int,
    Array,
    Bool,
}

impl Type {
    pub fn new(type_str: &str) -> Self {
        match type_str {
            "int" => Self::Int,
            "bool" => Self::Bool,
            _ => unreachable!(),
        }
    }
}
