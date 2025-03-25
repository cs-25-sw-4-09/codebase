
use super::grammar::context_free_grammar;
use hime_redist::ast::AstNode;

#[test]
fn test_1() {

    
    let result = context_free_grammar::parse_str(str);
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