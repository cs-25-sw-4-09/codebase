use lexer_parser::{
    utils::{
        tree_converter::stringify_tree, 
      //  tree_builder::TreeBuilderStr
    }, 
    grammar::cfg,
    //valid_programs::get_programs
};

fn main() {
    let program = "
    import xy \"hello\";
    y: int;
    begin
    x: int[] = [1, 2];
    y: int = x[0];
    x = [ ];
    ";

    println!("{}", stringify_tree(cfg::parse_str(program).get_ast().get_root()))
    
}

