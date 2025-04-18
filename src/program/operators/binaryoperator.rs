use std::error::Error;

use hime_redist::symbols::Symbol;

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
    pub fn new(operator: Symbol) -> Result<Self, Box<dyn Error>> {
        let operator = match operator.name {
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
        };
        Ok(operator)
    }
}