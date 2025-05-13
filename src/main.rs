use std::{env, error::Error, path::Path};

use codebase::{
    generators::generator::{self, get_generator},
    interpreter::{value::Value, InterpretE, InterpretP},
    lexer_parser::{
        grammar::cfg,
        //valid_programs::get_programs
        utils::tree_converter::stringify_tree,
    },
    program::{expression::Expr, program::Program, statement::Stmt},
    typechecker::{TypeCheckE, TypeCheckP},
};

fn main() -> Result<(), Box<dyn Error>> {
    //Lexer/Parser

    let mut args = env::args();
    let executable_name = args.next().unwrap();
    let file_to_parse = args.next().ok_or_else(|| {
        format!(
            "{} <input> <generator> [-argument value] [-argument2 value2]...",
            executable_name
        )
    })?;
    let file_stem = file_to_parse.rsplitn(2, '.').last().unwrap();


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
                            "[Typechecker] Command line argument with name {}'s value could not be interpreted.",
                            identifier
                        ));
                        })?;
                        program.ienvironment.vtable_push(identifier.into(), i1);
                    }
                    false => {
                        return Err(format!(
                            "[Typechecker] Command line argument with name {}'s value is not of the correct type: {:?}.",
                            identifier, program_argument_type
                        )
                        .into());
                    }
                },
                Err(_) => {
                    return Err(format!(
                        "[Typechecker] Command line argument with name {}'s value could not be type checked.",
                        identifier
                    )
                    .into());
                }
            },
            Err(_) => {
                return Err(
                    format!("[Typechecker] Command line argument with name {} not allowed", identifier).into(),
                );
            }
        }
    }
    program.tenvironment.clear();
    println!("[Typechecker] OK");

    //Interpret
    match program.interpret() {
        Ok(_) => println!("[Interpreter] OK"),
        Err(err) => {
            return Err(format!("[Interpreter] error: {}", err).into())
        }
    }

    program.ienvironment.darray_get_mut().flip_y();
    //Generate Files from draw array
    for output_generator in output_generators {
        if let Some(generator) = get_generator(&output_generator) {
            if let Err(err) =
                generator.generate(program.ienvironment.darray_get(), file_stem.into())
            {
                println!(
                    "Failed to generate format: {}, err: {}",
                    output_generator, err
                );
            }
        } else {
            println!("Unsupported format: {}", output_generator);
        }
    }

    Ok(())
}
