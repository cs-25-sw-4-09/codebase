use crate::program::statement::Stmt;

use super::{InterpretE, InterpretS};

impl InterpretS for Stmt {
    fn interpret(
        &self,
        environment: &mut super::environment::IEnvironment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Stmt::VarDecl {
                name,
                declared_type,
                value,
            } => {
                
                let i1 = value.interpret(environment)?;

                environment.vtable_push(name.into(), i1)
            },
            Stmt::FuncDecl {
                name,
                return_type,
                parameters,
                statements,
            } => todo!(),
            Stmt::Return(expr) => todo!(),
            Stmt::Decl {
                name,
                declared_type,
                value,
            } => todo!(),
            Stmt::Import { name, path } => todo!(),
        }

        Ok(())
    }
}
