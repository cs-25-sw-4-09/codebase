use crate::{
    interpreter::{environment::IEnvironment, value::Value, InterpretE}, 
    program::expression::Expr
};
use std::ops;

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


impl TryFrom<(&Expr, &mut IEnvironment)> for Point {
    type Error = Box<dyn std::error::Error>;
    
    fn try_from(all: (&Expr, &mut IEnvironment)) -> Result<Self, Self::Error> {
        let (value, env) = all;
        match value {
            Expr::Point(x, y) => {
                let (x, y) = (x.interpret(env)?, y.interpret(env)?);
                Ok(Self { x: Box::new(x), y: Box::new(y) })
            }, 
            _ => todo!(),
        } 
    }
}

impl Point { 
    pub fn x(&self) -> &Value{
        &self.x
    }
    
    pub fn y(&self) -> &Value{
        &self.y
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let new_x = *self.x + *rhs.x;
        let new_y = *self.y + *rhs.y;
        Self { 
            x: Box::new(new_x),
            y: Box::new(new_y)
        }
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_x = *self.x - *rhs.x;
        let new_y = *self.y - *rhs.y;
        Self { 
            x: Box::new(new_x),
            y: Box::new(new_y)
        }
    }
}