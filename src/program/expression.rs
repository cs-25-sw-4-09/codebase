use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::{environment::TEnvironment, operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator}, r#type::Type};

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
    pub fn new(expr: AstNode) -> Self {
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

    pub fn type_check(&self, environment: &mut TEnvironment) -> Result<Type, ()> {
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