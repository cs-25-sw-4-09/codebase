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
        z: bool = x == 1;
        f: bool = y && z;
        return f;
    }
    x: bool = funktionsnavn(5 * (3 + 2), true && true); //Error:: du kan ikke skrive true == true e.g.
    ";

    println!(
        "{}",
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
    let mut f = Program::new(cfg::parse_str(program).get_ast().get_root());
    f.type_check();
    println!("{:?}", f);
}
