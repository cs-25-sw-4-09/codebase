use std::error::Error;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
    Bool,
    Float,
    Shape,
    Color
}

impl Type {
    pub fn new(type_str: &str) -> Result<Self, Box<dyn Error>> {
        let r#type = match type_str {
            "int" => Self::Int,
            "bool" => Self::Bool,
            "float" => Self::Float,
            "shape" => Self::Shape,
            "color" => Self::Color,
            _ => unreachable!(),
        };
        Ok(r#type)
    }
}