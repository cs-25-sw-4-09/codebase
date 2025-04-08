use lexer_parser::{
    grammar::cfg, types::types::Program, utils::{
        tree_builder::TreeBuilderStr, tree_converter::stringify_tree
    }, valid_programs::get_programs
};

fn main() {
    

    let program = "
    begin
    x: bool = false;
    y: int = 3 + x;
    ";

    println!("{}", stringify_tree(cfg::parse_str(program).get_ast().get_root()));
    let f = Program::new(cfg::parse_str(program).get_ast().get_root());
    println!("{:?}", f);
    
}

