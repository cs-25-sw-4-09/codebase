use std::{env, error::Error, path::Path};

use codebase::{
    generators::generator::{self, get_generator},
    interpreter::InterpretP,
    lexer_parser::{
        grammar::cfg,
        //valid_programs::get_programs
        utils::tree_converter::stringify_tree,
    },
    program::program::Program,
    typechecker::TypeCheckP,
};

fn main() -> Result<(), Box<dyn Error>> {
    //Lexer/Parser
    let mut program = Program::from_file(Path::new("main.extension"))?;

    //Typecheck
    if let Err(err) = program.type_check() {
        println!("[Typechecker] error: {}", err);
        return Err(err);
    }
    program.tenvironment.clear();
    println!("[Typechecker] OK");

    //Interpret
    match program.interpret() {
        Ok(_) => println!("[Interpreter] OK"),
        Err(err) => println!("[Interpreter] error: {}", err),
    }

    //Generate Files from draw array

    let format = "svg";

    if let Some(generator) = get_generator(format) {
        if let Err(err) = generator.generate(program.ienvironment.darray_get()) {
            println!("Failed to generate format: {}, err: {}", format, err);
        }
    } else {
        println!("Unsupported format: {}", format);
    }


    //println!("{:#?}", program.ienvironment);

    Ok(())
}
