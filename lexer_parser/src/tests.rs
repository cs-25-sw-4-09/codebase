
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


/* Expressions */
#[test]
fn test_multi_and_add_precedence() {
    let part_program = vec![ ("program", 0), ("declS", 1), ("begin", 1), ("stmtS", 1), ("stmt", 2), ("_", 3), ("=", 3)].into_iter();
    let part_program2 = vec![(";", 3)].into_iter();
    
    let expr = vec![ ("+", 3), ("5", 4), ("*", 4), ("5", 5), ("6", 5)].into_iter();
    let nodes = part_program.clone().chain(expr).chain(part_program2.clone()).collect();

    test_equality(
        nodes, 
        "begin _ = 5 + 5 * 6;"
    );

    let expr = vec![ ("+", 3), ("*", 4), ("1", 5), ("2", 5), ("3", 4)].into_iter();
    let nodes = part_program.chain(expr).chain(part_program2).collect();
    test_equality(
        nodes, 
        "begin _ = 1 * 2 + 3;"
    );
}

#[test]
fn multi_stmt() {
    let program = "begin _ = 5; _ = 5; _ = 5; _ = 5; _ = 5; _ = 5;";
    let stmt = vec![("stmt", 2), ("_", 3), ("=", 3), ("5", 3), (";", 3)];
    let stmt = stmt.iter().cycle().take(stmt.len() * 6).cloned();

    test_equality(
        vec![("program", 0),("declS", 1),("begin", 1),("stmtS", 1)].into_iter().chain(stmt).collect::<Vec<_>>(), 
        program
    );
}

#[test]
fn function_declaration() {
    let program = 
    "begin
    x(y: int, z: int): int -> {
        return y+z;
    } 
    a: int = x(5, 6);
    ";
    let mut start = vec![
        ("program",0), ("declS",1), ("begin", 1), ("stmtS", 1),
    ];
    let stmt1 = vec![("stmt", 2), 
    ("x", 3), ("(", 3), ("params", 3), 
    ("param", 4), ("y", 5), (":", 5), ("int", 5), 
    (",", 4), 
    ("param", 4), ("z", 5), (":", 5), ("int", 5),
    (")",3), (":",3), ("int",3), ("->",3)
    ];
    let func_stmt = vec![
        ("{", 3), ("stmt", 3), ("return", 4), ("+", 4), ("y", 5), ("z", 5), (";", 4), ("}", 3)
    ];
    let stmt2 = vec![
        ("stmt", 2), ("a", 3), (":", 3), ("int", 3), ("=", 3), 
        ("FCall", 3),
        ("x", 4), ("(", 4), 
        ("args", 4), ("arg", 5), ("5", 6),(",", 5), ("arg", 5), ("6", 6),
        (")", 4), (";", 3)
    ];
    start.extend(stmt1.iter());
    start.extend(func_stmt.iter());
    start.extend(stmt2.iter());
    test_equality(
        start, 
        program
    );
}

#[test]
fn stmt_decl_assign() {
    let program = "
    begin
    x: bool = true;
    x = !x;
    ";
    let stmts = vec![
        ("stmt", 2), ("x", 3), (":", 3), ("bool", 3), ("=", 3), ("true", 3), (";", 3),
        ("stmt", 2), ("x", 3), ("=", 3), ("false", 3), (";", 3)
    ];
    let mut nodes = vec![
        ("program", 0), ("declS", 1), ("begin", 1), ("stmtS", 1)
    ];
    nodes.extend(stmts.iter());

    test_equality(nodes, program);
}


#[test]
fn fork_single() {
    let program= "
    begin 
    fork {
        (5 > 4) -> { return 10; }
    }";

    let mut start = vec![("program", 0), ("declS",1), ("begin", 1), ("stmtS", 1)];
    let stmt1 = vec![
        ("stmt", 2), 
        ("fork", 3), ("{", 3), ("fork", 3),
        ("(", 4), (">",4),
        ("5", 5), ("4", 5), (")", 4), ("->", 4), ("{", 4), ("stmtS", 4), ("stmt", 5), ("return", 6), ("10", 6), (";", 6), ("}", 4), ("}", 3)];
    start.extend(stmt1.iter());

    test_equality(
        start, 
        program
    );
}

#[test]
fn fork_multi() {
    let program= "
    begin 
    fork { 
        fork {5 == 5} -> { 5 + 5;}
        fork {5 > 3} -> {6 * 7;}
    }";

    let mut start = vec![("program", 0), ("declS",1), ("begin", 1), ("stmtS", 1)];
    let stmt1 = vec![
        ("stmt", 2), 
        ("fork", 3), ("{", 3), ("fork", 3),
        ("(", 4), (">",4),
        ("5", 5), ("4", 5), (")", 4), ("->", 4), ("{", 4), ("stmt", 4), ("return", 5), ("10", 5), (";", 5), ("}", 4), ("}", 3),
        ("5", 5), ("4", 5), (")", 4), ("->", 4), ("{", 4), ("stmtS", 4), ("stmt", 5), ("return", 6), ("10", 6), (";", 6), ("}", 4), ("}", 3)
    ];
    start.extend(stmt1.iter());

    test_equality(
        start, 
        program
    );  
}

