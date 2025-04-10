
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Bool,
    Float,
}

impl Type {
    pub fn new(type_str: &str) -> Self {
        match type_str {
            "int" => Self::Int,
            "bool" => Self::Bool,
            "float" => Self::Bool,
            _ => unreachable!(),
        }
    }
}