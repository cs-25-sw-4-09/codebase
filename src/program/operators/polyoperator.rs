use std::error::Error;

use hime_redist::symbols::Symbol;

#[derive(Debug, PartialEq, Clone)]
pub enum PolyOperator {
    Polygon,
}

impl PolyOperator {
    pub fn new(operator: Symbol) -> Result<Self, Box<dyn Error>> {
        let operator = match operator.name {
            "--*" => Self::Polygon,
            "~~*" => Self::Polygon,
            _ => panic!(),
        };
        Ok(operator)
    }
}
