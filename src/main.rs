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
    begin
    funktionsnavn(x: int, y: bool): bool -> {
        z: int = 5;
        return z;
    }
    ";

    println!(
        "{}",
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
    let mut f = Program::new(cfg::parse_str(program).get_ast().get_root());
    f.type_check();
    println!("{:?}", f);
}
