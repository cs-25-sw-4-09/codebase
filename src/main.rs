use codebase::{
    lexer_parser::{
        grammar::cfg,
        //valid_programs::get_programs
        utils::tree_converter::stringify_tree,
    },
    types::types::Program,
};

fn main() {
    let program = "
    begin
    x: int = 4;
    y: int = 2 + x % 3;
    ";

    println!(
        "{}",
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
    let mut f = Program::new(cfg::parse_str(program).get_ast().get_root());
    f.type_check();
    println!("{:?}", f);
}
