use codebase::{
    lexer_parser::{
        grammar::cfg,
        //valid_programs::get_programs
        utils::tree_converter::stringify_tree,
    },
    program::program::Program, typechecker::TypeCheckP,
};

fn main() {
    let program = "
    import circle \"circle.extension\";
    y: int;
    begin
    x: int = 5 + 7;
    ";

    println!("PROGRAM {}", program);

    println!(
        "{}",
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
    let mut f = Program::new(program);
    match f.type_check() {
        Ok(_) => println!("[Typechecker] OK"),
        Err(_) => println!("[Typechecker] ERROR"),
    }
    println!("{:?}", f);
}
