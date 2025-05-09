use std::error::Error;

use hime_redist::symbols::Symbol;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    Negate,
    Negative,
}

impl UnaryOperator {
    pub fn new(operator: Symbol) -> Result<Self, Box<dyn Error>> {
        let operator = match operator.name {
            "!" => Self::Negate,
            "-" => Self::Negative,
            _ => unreachable!(),
        };

        Ok(operator)
    }
}