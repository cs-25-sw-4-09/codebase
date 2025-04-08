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

    pub fn type_check(&self) {
        let mut vtable: HashMap<String, Type> = HashMap::new();
        self.stmts.iter().for_each(|stmt| {
            println!("{}", if stmt.type_check(&mut vtable).is_ok() { "TRUE" } else { "FALSE" })
        })
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

    fn type_check(&self, vtable: &mut HashMap<String, Type>) -> Result<Type, ()> {
        match self {
            Stmt::VarDecl { name, declared_type, value } => {
                if vtable.contains_key(name) { return Err(()) };
                if declared_type.eq(&value.type_check(vtable)?) {
                    vtable.insert(name.clone(), declared_type.clone()); //måske fiks clone here
                    return Ok(declared_type.clone());
                }
                Err(())
            },
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    Float(f64),
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
            "FLOAT" =>  Expr::Float(expr.get_value().unwrap().parse().unwrap()),
            _ => panic!("Expression type not found: {}", expr.get_symbol().name),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Expr::Integer(_) => Type::Int,
            Expr::Boolean(_) => Type::Bool,
            Expr::Float(_)  => Type::Float,
            christmas => panic!("{:?}", christmas)
        }
    }

    fn type_check(&self, vtable: &HashMap<String, Type>) -> Result<Type, ()> {
        match self {
            Expr::Integer(_) => Ok(Type::Int),
            Expr::Boolean(_) => Ok(Type::Bool),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Variable(identifier) => vtable.get(identifier).cloned().ok_or(()),
            Expr::BinaryOperation { lhs, rhs, operator: _ } => {
                let t1 = lhs.type_check(vtable)?;
                let t2 = rhs.type_check(vtable)?;
                match (t1, t2) {
                    (Type::Int, Type::Int) => Ok(Type::Int),
                    (Type::Float, Type::Float) => Ok(Type::Float),
                    (Type::Int, Type::Float) | (Type::Float, Type::Int) => Ok(Type::Float),
                    _ => Err(())
                }

            },
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
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Array,
    Bool,
    Float
}

impl Type {
    pub fn new(type_str: &str) -> Self {
        match type_str {
            "int" => Self::Int,
            "bool" => Self::Bool,
            "float" => Self::Bool,
            _ => unreachable!(),
        }
    }
}
