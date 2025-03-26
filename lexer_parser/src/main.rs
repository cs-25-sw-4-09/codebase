#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

#[allow(dead_code)]

use hime_redist::ast::AstNode;

use hime_redist::symbols::SemanticElementTrait;
use lexer_parser::grammar::context_free_grammar;
use lexer_parser::programs::{get_example, stingify_tree};


fn main() {
    let string = get_example(0); 

    let str = 
    "begin
    x = 5 + 5;
    ";

    let example = "
    begin
    x = 5 + 5;
    ";
    let result = context_free_grammar::parse_str(example);
    let ast = result.get_ast();
    let root = ast.get_root(); 
    print(root, &[]);

    //println!("{}", stingify_tree(root));

    let example = "
    begin
    x = 5 + 5 * 6;
    ";

}

fn print(node: AstNode, crossings: &[bool]) {

    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{node}");

    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.to_owned();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), &child_crossings);
        i += 1;
    }
}
