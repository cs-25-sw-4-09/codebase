use std::error::Error;


use data_types::figurearray::FigureArray;
use environment::IEnvironment;
use value::Value;

pub mod data_types;
pub mod utils;
pub mod expression;
pub mod program;
pub mod statement;
pub mod environment;
pub mod errors;
pub mod stack;
pub mod value;
mod tests;

pub trait InterpretP {
    fn interpret(&mut self) -> Result<&FigureArray, Box<dyn Error>>;
}
pub trait InterpretS {
    fn interpret(&self, environment: &mut IEnvironment) -> Result<(), Box<dyn Error>>;
}
pub trait InterpretE {
    fn interpret(&self, environment: &mut IEnvironment) -> Result<Value, Box<dyn Error>>;
}