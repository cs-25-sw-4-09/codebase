use std::error::Error;

use hime_redist::symbols::Symbol;

#[derive(Debug, PartialEq, Clone)]
pub enum PathOperator {
    Line,
    Curve,
}

impl PathOperator {
    pub fn new(operator: Symbol) -> Result<Self, Box<dyn Error>> {
        let operator = match operator.name {
            "--" => Self::Line,
            "~~" => Self::Curve,
            _ => panic!(),
        };
        Ok(operator)
    }
}
