use crate::program::statement::Stmt;

use super::InterpretS;

impl InterpretS for Stmt {
    fn interpret(&self, environment: &mut super::environment::IEnvironment) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}