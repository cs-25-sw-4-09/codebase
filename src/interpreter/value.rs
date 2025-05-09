
use super::data_types::{figure::Figure,point::Point};
use std::error::Error;

#[derive(Debug, Clone,PartialEq)]
pub enum Value {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    Float(f64),
    Point(Point),
    Color(Box<Value>, Box<Value>, Box<Value>, Box<Value>),
    Shape(Vec<Figure>),
    Figure(Figure),
    Array(Vec<Box<Value>>)
}

impl Value {
    pub fn get_int(&self) -> Result<i64, Box<dyn Error>> {
        match self {
            Value::Integer(i) => Ok(*i),
            _ => Err(crate::program::errors::ExprParseAsIntegerError.into())
        }
    }

    pub fn get_bool(&self) -> Result<bool, Box<dyn Error>> {
        match self {
            Value::Boolean(i) => Ok(*i),
            _ => Err(crate::program::errors::ExprParseAsBooleanError.into())
        }
    }
}