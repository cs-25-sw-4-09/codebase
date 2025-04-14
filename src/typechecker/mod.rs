use std::error::Error;

use environment::TEnvironment;

use crate::program::r#type::Type;

pub mod expression;
pub mod program;
pub mod statement;
pub mod environment;
pub mod errors;
mod tests;

pub trait TypeCheckE {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<Type, Box<dyn Error>>;
}
pub trait TypeCheckS {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<(), Box<dyn Error>>;
}
pub trait TypeCheckP {
    fn type_check(&mut self) -> Result<&TEnvironment, Box<dyn Error>>;
}