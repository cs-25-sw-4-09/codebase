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
    match program.type_check() {
        Ok(_) => println!("[Typechecker] OK"),
        Err(err) => println!("[Typechecker] error: {}", err),
    }

    match program.interpret() {
        Ok(_) => println!("[Interpreter] OK"),
        Err(err) => println!("[Interpreter] error: {}", err),
    }

    println!("{:?}", program);
    Ok(())
}
