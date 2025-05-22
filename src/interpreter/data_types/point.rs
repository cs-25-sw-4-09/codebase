use crate::interpreter::value::Value;
use std::ops::{self, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: Box::<Value>, 
    y: Box::<Value>
}

impl From<(Value, Value)> for Point {
    fn from(value: (Value, Value)) -> Self {
        Self { x: Box::new(value.0), y: Box::new(value.1) }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self { x: Box::new(Value::Integer(value.0)), y: Box::new(Value::Integer(value.1)) }
    }
}

impl From<(f64, f64)> for Point {
    fn from(value: (f64, f64)) -> Self {
        Self { x: Box::new(Value::Float(value.0)), y: Box::new(Value::Float(value.1)) }
    }
}




impl Point { 
    
    pub fn get_x(&self) -> &Value{ &self.x }

    pub fn get_y(&self) -> &Value{ &self.y }

    pub fn set_x(&mut self, value: Value) { self.x = Box::new(value);
    }

    pub fn set_y(&mut self, value: Value) { self.y = Box::new(value); }

    pub fn approx_eq(&self, other: &Point, epsilon: f64) -> bool {
        self.get_x().approx_eq(other.get_x(), epsilon) &&
        self.get_y().approx_eq(other.get_y(), epsilon)
    }

    pub fn get_x_f64(&self) -> f64 {
        match self.x.as_ref() {
            Value::Integer(x) => *x as f64,
            Value::Float(x) => *x,
            _ => unreachable!()
        }
    }
    pub fn get_y_f64(&self) -> f64 {
        match self.y.as_ref() {
            Value::Integer(y) => *y as f64,
            Value::Float(y) => *y,
            _ => unreachable!()
        }
    }

    pub fn svg_format(&self) -> String {
        let (x, y) = match (self.get_x(), self.get_y()) {
        (Value::Integer(x), Value::Integer(y)) => (*x as f64, *y as f64),
        (Value::Integer(x), Value::Float(y)) => (*x as f64, *y),
        (Value::Float(x), Value::Integer(y)) => (*x, *y as f64),
        (Value::Float(x), Value::Float(y)) => (*x, *y),
        _ => unreachable!(),
        };
        format!("{},{}", x, y)
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self { 
            x: Box::new(*self.x + *rhs.x),
            y: Box::new(*self.y + *rhs.y)
        }
    }
}

impl ops::Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { 
            x: Box::new(self.get_x() + rhs.get_x()),
            y: Box::new(self.get_y() + rhs.get_y())
        }
    }
}


impl ops::Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { 
            x: Box::new(self.get_x() - rhs.get_x()),
            y: Box::new(self.get_y() - rhs.get_y())
        }
    }
}

impl ops::Div<&Value> for &Point {
    type Output = Point;

    fn div(self, rhs: &Value) -> Self::Output {
        Point {
            x: Box::new(self.get_x() / rhs),
            y: Box::new(self.get_y() / rhs),
        }
    }
}

impl Mul<&Value> for Point {
    type Output = Point;

    fn mul(self, rhs: &Value) -> Self::Output {
        Self {
            x: Box::new(self.get_x() * rhs),
            y: Box::new(self.get_y() * rhs),
        }
    }
}