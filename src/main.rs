use std::{env, error::Error, path::Path};

use codebase::{
    generators::generator::get_generator, interpreter::InterpretP, program::program::Program,
    typechecker::TypeCheckP,
};

fn main() -> Result<(), Box<dyn Error>> {
    //Lexer/Parser

    let mut args = env::args();
    let executable_name = args.next().unwrap(); // Will always exists, returns name of exetuable the program was executed using
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

    let mut program = Program::from_file(Path::new(file_to_parse.as_str()))
        .map_err(|err| format!("[Lexer/Parser] {}", err))?;
    program
        .type_check()
        .map_err(|err| format!("[Typechecker] {}", err))?;

    program.parse_terminal_args(args)?;
    program.tenvironment.clear();
    println!("[Typechecker] OK");

    //Interpret
    program
        .interpret()
        .map_err(|err| format!("[Interpreter] {}", err))?;
    println!("[Interpreter] OK");

    if let Some(returnvalue) = program.ienvironment.rvalue_get() {
        let exit_code = returnvalue.get_int()?;
        if exit_code != 0 {
            return Err(format!("[Interpreter] exited with error code: {}", exit_code).into());
        }
    }

    //Generate Files from draw array
    output_generators
        .into_iter()
        .filter_map(|gen_name| {
            let generator = get_generator(&gen_name);
            if generator.is_none() {
                println!("[Generator] Unsupported format: {}", gen_name);
            }
            Some(gen_name).zip(generator)
        })
        .for_each(|(gen_name, mut generator)| {
            if let Err(err) =
                generator.generate(program.ienvironment.darray_get().clone(), file_stem.into())
            {
                println!(
                    "[Generator] Failed to generate format: {}, err: {}",
                    gen_name, err
                );
            }
        });

    Ok(())
}
