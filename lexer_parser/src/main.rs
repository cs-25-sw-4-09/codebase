

use lexer_parser::{
    grammar::cfg,
    utils::tree_converter::{
        stringify_tree,
        print
    },
    valid_programs::get_programs
};


fn main() {
    get_programs().into_iter().for_each(|el| { let _ = cfg::parse_str(el).get_ast().get_root(); });

    let _ = 
    "
    import func \"hello\";
    import func2 \"world\";

    x: int;
    
    begin
    x = 5 * 5 + 6;
    y = x + 7;


    draw x;
    ";
    let example = "begin
    x= 5 + 6 * 7;";

    let result = cfg::parse_str(example);
    let ast = result.get_ast();
    let root = ast.get_root(); 
    print(root, &[]);

    let str = stringify_tree(root);
    println!("{}", str);

}

