use std::error::Error;


use environment::IEnvironment;

use crate::program::{expression::Expr, r#type::Type};

pub mod expression;
pub mod program;
pub mod statement;
pub mod environment;
pub mod errors;
pub mod stack;

pub trait InterpretE {
    fn interpret(&self, environment: &mut IEnvironment) -> Result<Expr, Box<dyn Error>>;
}
pub trait InterpretS {
    fn interpret(&self, environment: &mut IEnvironment) -> Result<(), Box<dyn Error>>;
}
pub trait InterpretP {
    fn interpret(&mut self) -> Result<&IEnvironment, Box<dyn Error>>;
}