use std::error::Error;

use hime_redist::symbols::Symbol;

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
}

impl UnaryOperator {
    pub fn new(operator: Symbol) -> Result<Self, Box<dyn Error>> {
        let operator = match operator.name {
            "!" => Self::Negate,
            _ => panic!(),
        };

        Ok(operator)
    }
}