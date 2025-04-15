use std::{collections::HashMap, error::Error};

use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator};

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    Float(f64),
    Color(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    BinaryOperation {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        operator: BinaryOperator,
    },
    UnaryOperation {
        operator: UnaryOperator,
        expr: Box<Expr>,
    },
    FCall {
        name: String,
        args: Vec<Expr>,
    },
    SCall {
        name: String,
        args: HashMap<String, Expr>,
    },
}

impl Expr {
    pub fn new(expr: AstNode) -> Result<Self, Box<dyn Error>> {
        let expr = match expr.get_symbol().name {
            "INTEGER" => Expr::Integer(expr.get_value().unwrap().parse().unwrap()),
            "+" | "-" | "*" | "/" | "%" | "<" | ">" | "<=" | ">=" | "!=" | "==" | "&&" | "||" => {
                let lhs = Box::new(Expr::new(expr.child(0))?);
                let rhs = Box::new(Expr::new(expr.child(1))?);
                let operator = BinaryOperator::new(expr.get_symbol())?;

                Expr::BinaryOperation { lhs, rhs, operator }
            }
            "!" => {
                let uexpr = Box::new(Expr::new(expr.child(0))?);
                let operator = UnaryOperator::new(expr.get_symbol())?;

                Expr::UnaryOperation {
                    operator,
                    expr: uexpr,
                }
            }
            "IDENTIFIER" => Expr::Variable(expr.get_value().unwrap().into()),
            "BOOLEAN" => Expr::Boolean(expr.get_value().unwrap().parse().unwrap()),
            "FLOAT" => Expr::Float(expr.get_value().unwrap().parse().unwrap()),
            "color" => Expr::Color(
                Box::new(Expr::new(expr.child(0))?),
                Box::new(Expr::new(expr.child(1))?),
                Box::new(Expr::new(expr.child(2))?),
                Box::new(Expr::new(expr.child(3))?)
            ),
            "FCall" => Expr::FCall {
                name: expr.child(0).get_value().unwrap().into(),
                args: expr
                    .child(1)
                    .children()
                    .iter()
                    .map(|arg| Expr::new(arg))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            "SCall" => Expr::SCall {
                name: expr.child(0).get_value().unwrap().into(),
                args: expr
                    .child(1)
                    .children()
                    .iter()
                    .map(|arg| {
                        let key: String = arg.child(0).get_value().unwrap().into();
                        let value = Expr::new(arg.child(1))?;
                        Ok::<(String, Expr), Box<dyn Error>>((key, value))
                    })
                    .collect::<Result<HashMap<_, _>, _>>()?,
            },
            _ => panic!("Expression type not found: {}", expr.get_symbol().name),
        };

        Ok(expr)
    }
}
