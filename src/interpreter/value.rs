
use super::data_types::{figure::Figure, figurearray::FigureArray, point::Point};
use std::{error::Error, ops::{Add, Neg, Sub},cmp::Ordering};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    Float(f64),
    Point(Point),
    Color(Box<Value>, Box<Value>, Box<Value>, Box<Value>),
    Shape(FigureArray),
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

    pub fn get_float(&self) -> Result<f64, Box<dyn Error>> {
        match self {
            Value::Integer(i) => Ok(*i as f64),
            Value::Float(f) => Ok(*f),
            _ => Err(crate::program::errors::ExprParseAsFloatError.into())
        }
    }


    pub fn get_bool(&self) -> Result<bool, Box<dyn Error>> {
        match self {
            Value::Boolean(i) => Ok(*i),
            _ => Err(crate::program::errors::ExprParseAsBooleanError.into())
        }
    }

    pub fn get_shape(self) -> Result<FigureArray, Box<dyn Error>> {
        match self {
            Value::Shape(i) => Ok(i),
            _ => Err(crate::program::errors::ExprParseAsBooleanError.into())
        }
    }

    pub fn get_point(self) -> Result<Point, Box<dyn Error>> {
        match self {
            Value::Point(i) => Ok(i),
            _ => Err(crate::program::errors::ExprParseAsBooleanError.into())
        }
    }
}


impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 + v2),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 + v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 + v2 as f64),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(v1 as f64 + v2),
            _ => unreachable!()
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
        (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 - v2),
        (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 - v2),
        (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 - v2 as f64),
        (Value::Integer(v1), Value::Float(v2)) => Value::Float(v1 as f64 - v2),
        _ => unreachable!()
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(i) => Value::Integer(-i),
            Value::Float(i) => Value::Float(-i),
            _ => unreachable!()
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Integer(v1), Value::Integer(v2)) => v1.cmp(v2),
            (Value::Float(v1), Value::Float(v2)) => v1.partial_cmp(v2).unwrap(),
            (Value::Float(v1), Value::Integer(v2)) => v1.partial_cmp(&(*v2 as f64)).unwrap(),
            (Value::Integer(v1), Value::Float(v2)) => (&(*v1 as f64)).partial_cmp(v2).unwrap(),
            _ => unreachable!()
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // delegate to your existing Ord implementation
    }
}


impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(v1), Value::Integer(v2)) => v1 == v2,
            (Value::Float(v1), Value::Float(v2)) => v1 == v2,
            (Value::Integer(v1), Value::Float(v2)) => (*v1 as f64) == *v2,
            (Value::Float(v1), Value::Integer(v2)) => *v1 == (*v2 as f64),
            _ => false,
        }
    }
}

impl Eq for Value {}