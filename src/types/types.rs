use std::collections::HashMap;

use hime_redist::{
    ast::AstNode,
    symbols::{SemanticElementTrait, Symbol},
};

#[derive(Debug)]
pub struct Program {
    pub decl_f: Vec<Decl>,
    pub stmts: Vec<Stmt>,
    pub tenvironment: TEnvironment,
}

#[derive(Debug)]
pub struct TEnvironment {
    v_table: HashMap<String, EType>,
    f_table: HashMap<String, Type>,
    s_table: HashMap<String, Type>,
    r_type: Option<Type>,
}

impl TEnvironment {
    fn new() -> Self {
        TEnvironment {
            v_table: HashMap::new(),
            f_table: HashMap::new(),
            s_table: HashMap::new(),
            r_type: None,
        }
    }

    fn vtable_lookup(&self, identifier: &String) -> Option<&Type> {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Normal(t) | EType::Decl(t) => Some(t),
            }
        } else {
            None
        }
    }

    fn vtable_set(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::Normal(r#type));
    }

    fn vdtable_lookup(&self, identifier: &String) -> Option<&Type> {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Decl(t) => Some(t),
                EType::Normal(_) => None,
            }
        } else {
            None
        }
    }

    fn vdtable_set(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::Decl(r#type));
    }

    fn vtable_contains(&self, identifier: &String) -> bool {
        self.v_table.contains_key(identifier)
    }

    fn vdtable_contains(&self, identifier: &String) -> bool {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Decl(_) => true,
                EType::Normal(_) => false,
            }
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub enum EType {
    Normal(Type),
    Decl(Type),
}

impl Program {
    pub fn new(root_node: AstNode) -> Self {
        let mut decl_f: Vec<Decl> = Vec::new();
        let mut stmts: Vec<Stmt> = Vec::new();
        let tenvironment = TEnvironment::new();

        for node in root_node.children() {
            match node.get_symbol().name {
                "declS" => {
                    for decl in node.children() {
                        todo!()
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

        Program {
            decl_f,
            stmts,
            tenvironment,
        }
    }

    pub fn type_check(&mut self) {
        self.stmts.iter().for_each(|stmt| {
            println!(
                "{}",
                if stmt.type_check(&mut self.tenvironment).is_ok() {
                    "TRUE"
                } else {
                    "FALSE"
                }
            )
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

    fn type_check(&self, environment: &mut TEnvironment) -> Result<Type, ()> {
        match self {
            Stmt::VarDecl {
                name,
                declared_type,
                value,
            } => {
                if environment.vtable_contains(name) {
                    return Err(());
                };
                if declared_type.eq(&value.type_check(environment)?) {
                    environment.vtable_set(name.clone(), declared_type.clone()); //måske fiks clone here
                    return Ok(declared_type.clone());
                }
                Err(())
            }
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
        operator: BinaryOperator,
    },
    UnaryOperation {
        operator: UnaryOperator,
        expr: Box<Expr>,
    },
}

impl Expr {
    fn new(expr: AstNode) -> Self {
        match expr.get_symbol().name {
            "INTEGER" => Expr::Integer(expr.get_value().unwrap().parse().unwrap()),
            "+" | "-" | "*" | "/" | "%" | "<" | ">" | "<=" | ">=" | "!=" | "==" | "&&" | "||" => {
                let lhs = Box::new(Expr::new(expr.child(0)));
                let rhs = Box::new(Expr::new(expr.child(1)));
                let operator = BinaryOperator::new(expr.get_symbol());

                Expr::BinaryOperation { lhs, rhs, operator }
            }
            "!" => {
                let uexpr = Box::new(Expr::new(expr.child(0)));
                let operator = UnaryOperator::new(expr.get_symbol());

                Expr::UnaryOperation {
                    operator,
                    expr: uexpr,
                }
            }
            "IDENTIFIER" => Expr::Variable(expr.get_value().unwrap().into()),
            "BOOLEAN" => Expr::Boolean(expr.get_value().unwrap().parse().unwrap()),
            "FLOAT" => Expr::Float(expr.get_value().unwrap().parse().unwrap()),
            _ => panic!("Expression type not found: {}", expr.get_symbol().name),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Expr::Integer(_) => Type::Int,
            Expr::Boolean(_) => Type::Bool,
            Expr::Float(_) => Type::Float,
            error => panic!("{:?}", error),
        }
    }

    fn type_check(&self, environment: &mut TEnvironment) -> Result<Type, ()> {
        match self {
            Expr::Integer(_) => Ok(Type::Int),
            Expr::Boolean(_) => Ok(Type::Bool),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Variable(identifier) => environment.vtable_lookup(identifier).cloned().ok_or(()),
            Expr::BinaryOperation { lhs, rhs, operator } => {
                let t1 = lhs.type_check(environment)?;
                let t2 = rhs.type_check(environment)?;
                match operator {
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Divide
                    | BinaryOperator::Multiply
                    | BinaryOperator::Modulus => match (t1, t2) {
                        (Type::Int, Type::Int) => Ok(Type::Int),
                        (Type::Float, Type::Float)
                        | (Type::Int, Type::Float)
                        | (Type::Float, Type::Int) => Ok(Type::Float),
                        _ => Err(()),
                    },
                    BinaryOperator::LessThan
                    | BinaryOperator::LessThanOrEquals
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::GreaterThanOrEquals
                    | BinaryOperator::NotEquals
                    | BinaryOperator::Equals => match (t1, t2) {
                        (Type::Int, Type::Int)
                        | (Type::Float, Type::Int)
                        | (Type::Int, Type::Float)
                        | (Type::Float, Type::Float) => Ok(Type::Bool),
                        _ => Err(()),
                    },
                    BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                        if t1.eq(&Type::Bool) && t2.eq(&Type::Bool) {
                            Ok(Type::Bool)
                        } else {
                            Err(())
                        }
                    }
                }
            }
            Expr::UnaryOperation { operator, expr } => {
                let t1 = expr.type_check(environment)?;
                match operator {
                    UnaryOperator::Negate => {
                        if t1.ne(&Type::Bool) {
                            Err(())
                        } else {
                            Ok(Type::Bool)
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    GreaterThanOrEquals,
    LessThanOrEquals,
    LessThan,
    GreaterThan,
    Equals,
    NotEquals,
    LogicalAnd,
    LogicalOr,
}

impl BinaryOperator {
    fn new(operator: Symbol) -> Self {
        match operator.name {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "%" => Self::Modulus,
            "<" => Self::LessThan,
            ">" => Self::GreaterThan,
            "<=" => Self::LessThanOrEquals,
            ">=" => Self::GreaterThan,
            "!=" => Self::NotEquals,
            "==" => Self::Equals,
            "&&" => Self::LogicalAnd,
            "||" => Self::LogicalOr,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
}

impl UnaryOperator {
    fn new(operator: Symbol) -> Self {
        match operator.name {
            "!" => Self::Negate,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Array,
    Bool,
    Float,
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
