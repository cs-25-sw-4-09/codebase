use super::utils::{tree_converter::stringify_tree, tree_builder::TreeBuilderStr};
use super::grammar::cfg;

/* Helper functions */
fn test_equality(nodes: Vec<(&str, usize)>, program: &str) {
    assert_eq!(
        convert_nodes(nodes),
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
}

fn convert_nodes(nodes: Vec<(&str, usize)>) -> String {
    nodes
    .into_iter() 
    .fold( TreeBuilderStr::new(), |builder: TreeBuilderStr, (content, indent)| builder.add(content, indent))
    .build()
}



#[test]
fn test_multi_and_add_precedence() {
    let nodes = vec![
        ("program", 0),
        ("decl", 1),
        ("begin", 1),
        ("stmtS", 1),
        ("stmt", 2),
        ("_", 3),
        ("=", 3),
        ("+", 3),
        ("5", 4),
        ("*", 4),
        ("5", 5),
        ("6", 5),
        (";", 3)];
    let program = "
    begin
    _ = 5 + 5 * 6;
    ";
    test_equality(nodes, program);
    let program = "
    begin
    _ = 1 * 2 + 3;
    ";
    let nodes = vec![
        ("program", 0),
        ("decl", 1),
        ("begin", 1),
        ("stmtS", 1),
        ("stmt", 2),
        ("_", 3),
        ("=", 3),
        ("+", 3),
        ("*", 4),
        ("1", 5),
        ("2", 5),
        ("3", 4),
        (";", 3)
    ];
    test_equality(nodes, program);
}

