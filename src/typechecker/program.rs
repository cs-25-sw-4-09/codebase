use crate::{program::{program::Program, r#type::Type}, typechecker::TypeCheckS};

use super::{environment::TEnvironment, TypeCheckP};

impl TypeCheckP for Program {
    fn type_check(&mut self) -> Result<&TEnvironment, ()> {
        self.tenvironment.r_type = Some(Type::Int);
        for stmt in self.decl_f.iter().clone() {
            stmt.type_check(&mut self.tenvironment)?
        }
        for stmt in self.stmts.iter().clone() {
            stmt.type_check(&mut self.tenvironment)?
        }
        return Ok(&self.tenvironment);
    }
}