use environment::TEnvironment;

use crate::program::r#type::Type;

pub mod expression;
pub mod program;
pub mod statement;
pub mod environment;

pub trait TypeCheckE {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<Type, ()>;
}
pub trait TypeCheckS {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<Type, ()>;
}
pub trait TypeCheckP {
    fn type_check(&mut self);
}