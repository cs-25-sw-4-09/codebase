#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

#[allow(dead_code)]

use hime_redist::ast::AstNode;

use lexer_parser::grammar::cfg;
use lexer_parser::programs::{get_example, stringify_tree, print};


fn main() {
    let _string = get_example(0); 

    let example = 
    "
    import func \"hello\";
    import func2 \"world\";

    x: int;
    
    begin
    x = 5 * 5 + 6;
    y = x + 7;


    draw x;
    ";
    let example = 
    "begin 
    x = 5 +5 * 5;";

    let result = cfg::parse_str(example);
    let ast = result.get_ast();
    let root = ast.get_root(); 
    print(root, &[]);

    let str = stringify_tree(root);
    println!("{}", str);

}

