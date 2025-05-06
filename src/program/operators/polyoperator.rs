use std::error::Error;

use hime_redist::symbols::Symbol;

#[derive(Debug, PartialEq, Clone)]
pub enum PolyOperator {
    Curved,
    Straight
}

impl PolyOperator {
    pub fn new(operator: Symbol) -> Result<Self, Box<dyn Error>> {
        let operator = match operator.name {
            "--*" => Self::Straight,
            "~~*" => Self::Curved,
            _ => panic!(),
        };
        Ok(operator)
    }
}
