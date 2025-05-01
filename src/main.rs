use std::{error::Error, path::Path};

use codebase::{
    interpreter::InterpretP, lexer_parser::{
        grammar::cfg,
        //valid_programs::get_programs
        utils::tree_converter::stringify_tree,
    }, program::program::Program, typechecker::TypeCheckP
};

fn main() -> Result<(), Box<dyn Error>> {
    let program = include_str!("../main.extension");
    println!(
        "{}",
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
    
    let mut program = Program::from_file(Path::new("main.extension"))?;
    if let Err(err) = program.type_check() {
        println!("[Typechecker] error: {}", err);
        return Err(err)
    }
    println!("[Typechecker] OK");
    match program.interpret() {
        Ok(_) => println!("[Interpreter] OK"),
        Err(err) => println!("[Interpreter] error: {}", err),
    }

    println!("{:?}", program);
    Ok(())
}
