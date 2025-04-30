use crate::program::expression::Expr;

use super::InterpretE;

impl InterpretE for Expr {
    fn interpret(&self, environment: &mut super::environment::IEnvironment) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}