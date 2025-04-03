
use super::{
    utils::{
        tree_converter::stringify_tree, 
        tree_builder::TreeBuilderStr
    }, 
    grammar::cfg
};

/* Helper functions */
fn test_equality(nodes: Vec<(&str, usize)>, program: &str) {
    assert_eq!(
        TreeBuilderStr::new().multi_add(&nodes).build(),
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
}

#[test]
fn test_multi_and_add_precedence() {
    let part_program = vec![ ("program", 0), ("decl", 1), ("begin", 1), ("stmtS", 1), ("stmt", 2), ("_", 3), ("=", 3), (";", 3) ];
    
    let expr = vec![ ("+", 3), ("5", 4), ("*", 4), ("5", 5), ("6", 5)];
    let nodes = part_program.iter().take(part_program.len() - 1).cloned().chain(expr.into_iter()).chain(part_program.last().cloned()).collect();

    test_equality(
        nodes, 
        "begin _ = 5 + 5 * 6;"
    );

    let expr = vec![ ("+", 3), ("*", 4), ("1", 5), ("2", 5), ("3", 4)];
    let nodes = part_program.iter().take(part_program.len() - 1).cloned().chain(expr.into_iter()).chain(part_program.last().cloned()).collect();
    test_equality(
        nodes, 
        "begin _ = 1 * 2 + 3;"
    );
}

#[test]
fn multi_stmt() {
    let program = "begin _ = 5; _ = 5; _ = 5; _ = 5; _ = 5; _ = 5;";
    let stmt = vec![("stmt", 2), ("_", 3), ("=", 3), ("5", 3), (";", 3)];
    let stmt_s = vec![("stmtS", 1)].into_iter()
    .chain(stmt.iter().cycle().take(stmt.len() * 6).cloned())
    .collect::<Vec<_>>();
    test_equality(
        vec![("program", 0),("decl", 1),("begin", 1)].into_iter().chain(stmt_s).collect(), 
        program
    );
}

