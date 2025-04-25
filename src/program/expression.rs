use std::{collections::HashMap, error::Error};

use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::{
    errors,
    operators::{
    binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
    unaryoperator::UnaryOperator},
};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    Float(f64),
    Point(Box<Expr>, Box<Expr>),
    Color(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    PathOperation {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        operator: PathOperator,
    },
    PolygonOperation {
        path: Box<Expr>,
        operator: PolyOperator,
    },
    Array(Vec<Expr>),
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
            "INTEGER" => Expr::Integer(
                expr.get_value()
                    .ok_or_else(|| errors::ASTNodeValueInvalid(expr.get_symbol().name.to_owned()))?
                    .parse()?,
            ),
            "+" | "-" | "*" | "/" | "%" | "<" | ">" | "<=" | ">=" | "!=" | "==" | "&&" | "||" => {
                if expr.children_count() != 2 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(2, expr.children_count()).into(),
                    );
                }
                let lhs = Box::new(Expr::new(expr.child(0))?);
                let rhs = Box::new(Expr::new(expr.child(1))?);
                let operator = BinaryOperator::new(expr.get_symbol())?;

                Expr::BinaryOperation { lhs, rhs, operator }
            }
            "!" => {
                if expr.children_count() != 1 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(1, expr.children_count()).into(),
                    );
                }
                let uexpr = Box::new(Expr::new(expr.child(0))?);
                let operator = UnaryOperator::new(expr.get_symbol())?;

                Expr::UnaryOperation {
                    operator,
                    expr: uexpr,
                }
            }
            "--" | "~~" => {
                let lhs = Box::new(Expr::new(expr.child(0))?);
                let rhs = Box::new(Expr::new(expr.child(1))?);
                let operator = PathOperator::new(expr.get_symbol())?;

                Expr::PathOperation { lhs, rhs, operator }
            }
            "--*" | "~~*" => {
                let path = Box::new(Expr::new(expr.child(0))?);
                let operator = PolyOperator::new(expr.get_symbol())?;

                Expr::PolygonOperation { path, operator }
            }
            "IDENTIFIER" => Expr::Variable(
                expr.get_value()
                    .ok_or_else(|| errors::ASTNodeValueInvalid(expr.get_symbol().name.to_owned()))?
                    .into(),
            ),
            "BOOLEAN" => Expr::Boolean(
                expr.get_value()
                    .ok_or_else(|| errors::ASTNodeValueInvalid(expr.get_symbol().name.to_owned()))?
                    .parse()?,
            ),
            "FLOAT" => Expr::Float(
                expr.get_value()
                    .ok_or_else(|| errors::ASTNodeValueInvalid(expr.get_symbol().name.to_owned()))?
                    .parse()?,
            ),
            "point" => Expr::Point(
                Box::new(Expr::new(expr.child(0))?),
                Box::new(Expr::new(expr.child(1))?),
            ),
            "array" => Expr::Array(   
                expr
                    .children()
                    .iter()
                    .map(|arg| Expr::new(arg))
                    .collect::<Result<Vec<_>, _>>()?
            ),
            "color" => {
                if expr.children_count() != 4 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(4, expr.children_count()).into(),
                    );
                }
                Expr::Color(
                    Box::new(Expr::new(expr.child(0))?),
                    Box::new(Expr::new(expr.child(1))?),
                    Box::new(Expr::new(expr.child(2))?),
                    Box::new(Expr::new(expr.child(3))?),
                )
            }
            "FCall" => {
                if expr.children_count() != 2 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(2, expr.children_count()).into(),
                    );
                }
                Expr::FCall {
                    name: expr
                        .child(0)
                        .get_value()
                        .ok_or_else(|| {
                            errors::ASTNodeValueInvalid(expr.child(0).get_symbol().name.to_owned())
                        })?
                        .into(),
                    args: expr
                        .child(1)
                        .children()
                        .iter()
                        .map(|arg| Expr::new(arg))
                        .collect::<Result<Vec<_>, _>>()?,
                }
            }
            "SCall" => {
                if expr.children_count() != 2 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(2, expr.children_count()).into(),
                    );
                }
                Expr::SCall {
                    name: expr.child(0).get_value().ok_or_else(|| errors::ASTNodeValueInvalid(expr.child(0).get_symbol().name.to_owned()))?.into(),
                    args: expr
                        .child(1)
                        .children()
                        .iter()
                        .map(|arg| {
                            let key: String = arg
                                .child(0)
                                .get_value()
                                .ok_or_else(|| {
                                    errors::ASTNodeValueInvalid(
                                        arg.child(0).get_symbol().name.to_owned(),
                                    )
                                })?
                                .into();
                            let value = Expr::new(arg.child(1))?;
                            Ok::<(String, Expr), Box<dyn Error>>((key, value))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?,
                }
            }
            _ => unreachable!(),
        };

        Ok(expr)
    }
}
