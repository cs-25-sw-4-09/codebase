use std::{env, error::Error, path::Path};

use codebase::{
    interpreter::{value::Value, InterpretE, InterpretP},
    program::{expression::Expr, program::Program, statement::Stmt},
    typechecker::{TypeCheckE, TypeCheckP},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let executable_name = args.next().unwrap();
    let file_to_parse = args.next().ok_or_else(|| {
        format!(
            "{} <input> <generator> [-argument value] [-argument2 value2]...",
            executable_name
        )
    })?;
    let output_generators: Vec<String> = args
        .next()
        .ok_or_else(|| {
            format!(
                "{} <input> <generator> [-argument value] [-argument2 value2]...",
                executable_name
            )
        })?
        .split(",")
        .map(str::to_string)
        .collect();

    let mut program = Program::from_file(Path::new(file_to_parse.as_str()))?;

    if let Err(err) = program.type_check() {
        println!("[Typechecker] error: {}", err);
        return Err(err);
    }
    println!("[Typechecker] OK");
    let mut checkprogramargs = String::from("begin\n");
    while let (Some(mut arg_name), Some(arg_value)) = (args.next(), args.next()) {
        if arg_name.starts_with("-") {
            // only parse if on correct form
            arg_name = arg_name.replacen("-", "", 1); // Remove -
            checkprogramargs.extend(format!("{} = {};\n", arg_name, arg_value).chars());
        }
    }

    checkprogramargs.extend("return 0;".to_string().chars());

    let mut checkprogramargs = Program::new(&checkprogramargs)?;
    checkprogramargs.stmts.pop();
    for stmt in &checkprogramargs.stmts {
        let Stmt::Assign {
            name: identifier,
            value: expr,
        } = stmt
        else {
            unreachable!()
        };
        match program.tenvironment.vdtable_lookup(identifier) {
            Ok(program_argument_type) => match expr.type_check(&mut program.tenvironment.clone()) {
                Ok(expr_type) => match expr_type == *program_argument_type {
                    true => {
                        // Add the interpreted expression to the interpreter environment before execution.
                        let i1 = expr.interpret(&mut program.ienvironment).or_else(|_| {
                            return Err(format!(
                            "Command line argument with name {}'s value could not be interpreted.",
                            identifier
                        ));
                        })?;
                        program.ienvironment.vtable_push(identifier.into(), i1);
                    }
                    false => {
                        return Err(format!(
                            "Command line argument with name {}'s value is not of the correct type: {:?}.",
                            identifier, program_argument_type
                        )
                        .into());
                    }
                },
                Err(_) => {
                    return Err(format!(
                        "Command line argument with name {}'s value could not be type checked.",
                        identifier
                    )
                    .into());
                }
            },
            Err(_) => {
                return Err(
                    format!("Command line argument with name {} not allowed", identifier).into(),
                );
            }
        }
    }
    program.tenvironment.clear();

    match program.interpret() {
        Ok(_) => println!("[Interpreter] OK"),
        Err(err) => println!("[Interpreter] error: {}", err),
    }

    println!("{:#?}", program.ienvironment);
    Ok(())
}
