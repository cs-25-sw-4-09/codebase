use std::error::Error;

use crate::{program::program::Program, typechecker::TypeCheckS};

use super::{environment::TEnvironment, TypeCheckP};

impl TypeCheckP for Program {
    fn type_check(&mut self) -> Result<&TEnvironment, Box<dyn Error>> {
        for stmt in self.decl_f.iter().clone() {
            stmt.type_check(&mut self.tenvironment)?
        }
        for stmt in self.stmts.iter().clone() {
            stmt.type_check(&mut self.tenvironment)?
        }
        return Ok(&self.tenvironment);
    }
}
