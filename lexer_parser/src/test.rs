
use super::programs::stingify_tree;
use super::grammar::context_free_grammar;
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

#[test]
fn test_stmt_declaration() {

    let example = "
    begin
    x: int = 5 + 5;
    ";
    
    let assertions = vec!["begin", "x", ":", "int", "=", "5", "+", "5", ";"];
    check_ast_equality(
        &context_free_grammar::parse_str(example).get_ast().get_root(), 
        &assertions, 
        &mut 0
    );
}

#[test]
fn test_multi_precedence() {
    let example = "
    begin
    _ = 5 + 5 * 6;
    ";

    let correct_tree = 
"  begin
      _
    =
        5
      +
          5
        *
          6
    ;
";

    let tree2 = stingify_tree(
        context_free_grammar::parse_str(example)
        .get_ast().get_root()
    );
    println!("{correct_tree}");
    assert_eq!(correct_tree, tree2);

}




fn check_ast_equality(node: &AstNode, asserts: &[&str], index: &mut usize) {    
    let children = node.children(); 
    /*if node.get_value().is_some() {
        assert_eq!(node.get_value().unwrap(), asserts[*index]);
        *index += 1; 
        return;
    }*/

    for child in children.iter() { 
        if child.get_value().is_some() {
            assert_eq!(child.get_value().unwrap(), asserts[*index]);
            *index += 1; 
            return;
        }
    }

    for child in children.iter() {
        check_ast_equality(&child, asserts, index);
    }    

    let nodes = node.children().into_iter().collect::<Vec<_>>();
    let mut i = 0;
    while i < nodes.len() {




    }


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