use codebase::{
    lexer_parser::{
        grammar::cfg,
        //valid_programs::get_programs
        utils::tree_converter::stringify_tree,
    },
    program::program::Program, typechecker::TypeCheckP,
};

fn main() {
    let program = include_str!("../main.extension");
    println!(
        "{}",
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
    
    let mut program = Program::new("main.extension");
    match program.type_check() {
        Ok(_) => println!("[Typechecker] OK"),
        Err(_) => println!("[Typechecker] ERROR"),
    }
    println!("{:?}", program);
}
