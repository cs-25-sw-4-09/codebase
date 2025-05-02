use crate::program::program::Program;

use super::{environment::IEnvironment, InterpretP, InterpretS};

impl InterpretP for Program {
    fn interpret(&mut self) -> Result<&IEnvironment, Box<dyn std::error::Error>> {
        for stmt in self.decl_f.iter().clone() {
            stmt.interpret(&mut self.ienvironment)?
        }
        for stmt in self.stmts.iter().clone() {
            stmt.interpret(&mut self.ienvironment)?
        }
        Ok(&self.ienvironment)
    }
}
