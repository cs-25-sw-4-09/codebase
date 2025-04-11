use std::error::Error;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
    Bool,
    Float,
    Shape,
}

impl Type {
    pub fn new(type_str: &str) -> Result<Self, Box<dyn Error>> {
        let r#type = match type_str {
            "int" => Self::Int,
            "bool" => Self::Bool,
            "float" => Self::Bool,
            "shape" => Self::Shape,
            _ => unreachable!(),
        };
        Ok(r#type)
    }
}