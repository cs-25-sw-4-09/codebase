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
                declared_type: _,
                value,
            } => {
                let i1 = value.interpret(environment)?;

                environment.vtable_push(name.into(), i1)
            }
            Stmt::FuncDecl {
                name,
                return_type: _,
                parameters,
                statements,
            } => {
                environment.ftable_push(
                    name.into(),
                    statements.clone(),
                    parameters.iter().map(|p| p.0.clone()).collect(),
                );
            }
            Stmt::Return(expr) => {
                let i1 = expr.interpret(environment)?;
                environment.rvalue_set(i1);
            }
            Stmt::Decl {
                name,
                declared_type,
                value,
            } => todo!(),
            Stmt::Import { name, path } => todo!(),
            Stmt::Draw { shape, point } => todo!(),
            Stmt::Assign { name, value } => todo!(),
            Stmt::For {
                counter,
                from,
                to,
                body,
            } => todo!(),
            Stmt::Fork { branch, otherwise } => todo!(),
        }

        Ok(())
    }
}
