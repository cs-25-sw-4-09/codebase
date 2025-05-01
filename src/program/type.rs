use std::error::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
    Bool,
    Float,
    Shape,
    Point,
    Color,
    Path,
    Polygon,
    IntArray,
    BoolArray,
    FloatArray,
    ShapeArray,
    PointArray,
    ColorArray,
    PathArray,
    PolygonArray,
    Empty,
}

impl Type {
    pub fn new(type_str: &str) -> Result<Self, Box<dyn Error>> {
        let r#type = match type_str {
            "int" => Self::Int,
            "bool" => Self::Bool,
            "float" => Self::Float,
            "shape" => Self::Shape,
            "point" => Self::Point,
            "color" => Self::Color,
            "path" => Self::Path,
            "polygon" => Self::Polygon,
            "int[]" => Self::IntArray,
            "bool[]" => Self::BoolArray,
            "float[]" => Self::FloatArray,
            "shape[]" => Self::ShapeArray,
            "point[]" => Self::PointArray,
            "color[]" => Self::ColorArray,
            "path[]" => Self::PathArray,
            "polygon[]" => Self::PolygonArray,
            _ => unreachable!(),
        };
        Ok(r#type)
    }
}
