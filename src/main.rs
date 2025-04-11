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
    import circle \"./circle.extension\";
    begin
    x: int = 5;
    //draw circle(|width = 5|);
    //circle: shape = circle(|width = 4|);
    ";

    println!("PROGRAM {}", program);

    println!(
        "{}",
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
    let mut f = Program::new(program);
    f.type_check();
    println!("{:?}", f);
}
