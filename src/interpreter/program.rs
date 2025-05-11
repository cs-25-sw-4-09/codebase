use crate::program::program::Program;

use super::{data_types::figurearray::FigureArray, InterpretP, InterpretS};

impl InterpretP for Program {
    fn interpret(&mut self) -> Result<&FigureArray, Box<dyn std::error::Error>> {
        for stmt in self.decl_f.iter().clone() {
            stmt.interpret(&mut self.ienvironment)?
        }
        for stmt in self.stmts.iter().clone() {
            stmt.interpret(&mut self.ienvironment)?
        }
        Ok(self.ienvironment.darray_get())
    }
}
