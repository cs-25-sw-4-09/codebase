#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

#[allow(dead_code)]

use hime_redist::ast::AstNode;

use test_hime::grammar::math_exp;
use test_hime::programs::get_example;


fn main() {

    let string = get_example(0); 
    let string2 = 
    "import \"math\";
    begin 
    x: int = 5 + 5;"; 
    let string3 = "5 + 4";
    let result = math_exp::parse_str(string3);
    let ast = result.get_ast();
    let root = ast.get_root(); 
    print(root, &[]);
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