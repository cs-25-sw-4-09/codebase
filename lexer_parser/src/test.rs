
use super::programs::stringify_tree;
use super::grammar::context_free_grammar;

#[test]
fn test_stmt_declaration() {

    let _example = "
    begin
    x: int = 5 + 5;
    ";
    
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

    let tree2 = stringify_tree(
        context_free_grammar::parse_str(example)
        .get_ast().get_root()
    );
    println!("{correct_tree}");
    assert_eq!(correct_tree, tree2);

}



/*
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
*/