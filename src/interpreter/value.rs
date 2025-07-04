use super::data_types::{figure::Figure, figurearray::FigureArray, point::Point};
use std::{
    cmp::Ordering,
    error::Error,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    Float(f64),
    Point(Point),
    Color(Box<Value>, Box<Value>, Box<Value>, Box<Value>),
    Shape(FigureArray),
    Figure(Figure),
    Array(Vec<Value>),
}

impl Value {
    pub fn get_int(&self) -> Result<i64, Box<dyn Error>> {
        match self {
            Value::Integer(i) => Ok(*i),
            _ => Err(crate::program::errors::ExprParseAsIntegerError.into()),
        }
    }

    pub fn get_float(&self) -> Result<f64, Box<dyn Error>> {
        match self {
            Value::Integer(i) => Ok(*i as f64),
            Value::Float(f) => Ok(*f),
            _ => Err(crate::program::errors::ExprParseAsFloatError.into()),
        }
    }

    pub fn get_array(&self) -> Result<Vec<Value>, Box<dyn Error>> {
        match self {
            Value::Array(i) => Ok(i.clone()),
            _ => Err(crate::program::errors::ExprParseAsFloatError.into()),
        }
    }

    pub fn get_bool(&self) -> Result<bool, Box<dyn Error>> {
        match self {
            Value::Boolean(i) => Ok(*i),
            _ => Err(crate::program::errors::ExprParseAsBooleanError.into()),
        }
    }

    pub fn get_shape(self) -> Result<FigureArray, Box<dyn Error>> {
        match self {
            Value::Shape(i) => Ok(i),
            _ => Err(crate::program::errors::ExprParseAsShapeError.into()),
        }
    }

    pub fn get_point(self) -> Result<Point, Box<dyn Error>> {
        match self {
            Value::Point(i) => Ok(i),
            _ => Err(crate::program::errors::ExprParseAsPointError.into()),
        }
    }

    pub fn approx_eq(&self, other: &Value, epsilon: f64) -> bool {
        let (i1, i2) = match (self, other) {
            (Value::Integer(v1), Value::Integer(v2)) => (*v1 as f64, *v2 as f64),
            (Value::Float(v1), Value::Float(v2)) => (*v1, *v2),
            (Value::Float(v1), Value::Integer(v2)) => (*v1, *v2 as f64),
            (Value::Integer(v1), Value::Float(v2)) => (*v1 as f64, *v2),
            _ => unreachable!(),
        };
        (i1 - i2).abs() < epsilon
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
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
            _ => unreachable!(),
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
            _ => unreachable!(),
        }
    }
}

impl Add for &Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 + v2),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 + v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 + *v2 as f64),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(*v1 as f64 + v2),
            _ => unreachable!(),
        }
    }
}

impl Sub for &Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 - v2),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 - v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 - *v2 as f64),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(*v1 as f64 - v2),
            _ => unreachable!(),
        }
    }
}

impl Mul for &Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 * v2),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 * v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 * *v2 as f64),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(*v1 as f64 * v2),
            p => unreachable!("{:?}", p),
        }
    }
}

impl Div for &Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 / v2),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 / v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 / *v2 as f64),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(*v1 as f64 / v2),
            _ => unreachable!(),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(i) => Value::Integer(-i),
            Value::Float(i) => Value::Float(-i),
            _ => unreachable!(),
        }
    }
}

impl Neg for &Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(i) => Value::Integer(-i),
            Value::Float(i) => Value::Float(-i),
            _ => unreachable!(),
        }
    }
}

impl Not for Value {
    type Output = Value;
    fn not(self) -> Self::Output {
        match self {
            Value::Boolean(b) => Value::Boolean(!b),
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
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Value {}
