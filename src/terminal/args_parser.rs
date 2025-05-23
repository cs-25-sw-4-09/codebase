use std::env::Args;

use crate::{
    interpreter::InterpretE,
    program::{program::Program, statement::Stmt},
    typechecker::TypeCheckE,
};

impl Program {
    pub fn parse_terminal_args(&mut self, mut args: Args) -> Result<(), Box<dyn std::error::Error>> {
        let mut checkprogramargs = String::from("begin\n");
        while let (Some(mut arg_name), Some(arg_value)) = (args.next(), args.next()) {
            if arg_name.starts_with("-") {
                // only parse if on correct form
                arg_name = arg_name.replacen("-", "", 1); // Remove -
                checkprogramargs.extend(format!("{} = {};\n", arg_name, arg_value).chars());
            }
        }
        checkprogramargs.push_str("return 0;");

        let mut checkprogramargs =
            Program::new(&checkprogramargs).map_err(|err| format!("[Lexer/Parser] {}", err))?;

        checkprogramargs.stmts.pop(); //Removes the return statement
        for stmt in &checkprogramargs.stmts {
            let Stmt::Assign {
                name: identifier,
                value: expr,
            } = stmt
            else {
                unreachable!()
            };
            // Fetch the correct type for the command line flag
            let argument_type = *self.tenvironment.vdtable_lookup(identifier).map_err(|_| {
                format!(
                    "[Typechecker] Command line argument with name {} not allowed",
                    identifier
                )
            })?;

            // Get the type of the command line flag's expr value
            let expr_type = expr.type_check(&mut self.tenvironment).map_err(|_| 
                format!(
                    "[Typechecker] Command line argument with name {}'s value could not be type checked.",
                    identifier
                )
            )?;

            // Evaluate whether the command line flag's type equals the correct type
            if expr_type == argument_type {
                // Add the interpreted expression to the interpreter environment before execution.
                let i1 = expr.interpret(&mut self.ienvironment).map_err(|_| {
                    format!(
                        "[Typechecker] Command line argument with name {}'s value could not be interpreted.",
                        identifier
                    )
                })?;
                // Add the interpreted value to the interpreter environment
                self.ienvironment.vtable_push(identifier.into(), i1);        
            } else {
                return Err(format!(
                    "[Typechecker] Command line argument with name {}'s value is not of the correct type: {:?}.",
                    identifier, argument_type
                )
                .into());
            }
        }
        Ok(())
    }
}
