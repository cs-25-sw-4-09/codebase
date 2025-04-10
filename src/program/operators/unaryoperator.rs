use hime_redist::symbols::Symbol;

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
}

impl UnaryOperator {
    pub fn new(operator: Symbol) -> Self {
        match operator.name {
            "!" => Self::Negate,
            _ => panic!(),
        }
    }
}