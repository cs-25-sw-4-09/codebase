use crate::{
    interpreter::{environment::IEnvironment, value::Value, InterpretE}, 
    program::expression::Expr
};

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