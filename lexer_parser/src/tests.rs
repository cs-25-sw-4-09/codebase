
use super::{
    utils::{
        tree_converter::stringify_tree, 
        tree_builder::TreeBuilderStr
    }, 
    grammar::cfg,
    valid_programs::get_programs
};

/* Helper functions */
fn test_equality(nodes: Vec<(&str, usize)>, program: &str) {
    assert_eq!(
        TreeBuilderStr::new().multi_add(&nodes).build(),
        stringify_tree(cfg::parse_str(program).get_ast().get_root())
    );
}

/* Example Program parsing */
#[test]
fn example_program_parsing() {
    //A program is succesfully parsed if get_root() does not panic.
    get_programs()
    .into_iter()
    .for_each(|el| { 
        let _ = cfg::parse_str(el).get_ast().get_root(); 
    });
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
        ("{", 3), ("stmtS", 3), ("stmt", 4), ("return", 5), ("+", 5), ("y", 6), ("z", 6), (";", 5), ("}", 3)
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
    let stmt_l = 2;
    let stmts = vec![
        ("stmt", stmt_l), ("x", stmt_l + 1), (":", stmt_l + 1), ("bool", stmt_l + 1), ("=", stmt_l + 1), ("true", stmt_l + 1), (";", stmt_l + 1),
        ("stmt", stmt_l), ("x", stmt_l + 1), ("=", stmt_l + 1), ("!", stmt_l + 1), ("x", stmt_l + 2), (";", stmt_l + 1)
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
        ("fork", 3), ("{", 3), ("forkS", 3),
        ("fork", 4),("(", 5), (">",5),
        ("5", 6), ("4", 6), (")", 5), ("->", 5), ("{", 5), ("stmtS", 5), ("stmt", 6), ("return", 7),
        ("10", 7), (";", 7), ("}", 5), ("}", 3)];
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
        (5 == 5) -> { return 5 + 5;}
        (5 > 3) -> { x: int = 6 * 7; return x;} 
    }
";
    let mut start = vec![("program", 0), ("declS", 1), ("begin", 1), ("stmtS", 1)];
    let stmt_depth = 2;
    let stmt1 = vec![("stmt", stmt_depth), ("fork", stmt_depth+1), ("{", stmt_depth+1), 
    ("forkS", stmt_depth+1), 
    ("fork", stmt_depth+2), ("(", stmt_depth+3), ("==", stmt_depth+3), 
    ("5", stmt_depth+4), ("5", stmt_depth+4), (")", stmt_depth+3), ("->", stmt_depth+3), ("{", stmt_depth+3),
    ("stmtS", stmt_depth+3),("stmt", stmt_depth+4), ("return", stmt_depth+5), ("+", stmt_depth+5), ("5", stmt_depth+6),
     ("5", stmt_depth+6), (";", stmt_depth+5), ("}", stmt_depth+3)
    ];

    let stmt2 = vec![("fork", stmt_depth+2), ("(", stmt_depth+3), (">", stmt_depth+3), 
    ("5", stmt_depth+4), ("3", stmt_depth+4), (")", stmt_depth+3), ("->", stmt_depth+3), ("{", stmt_depth+3),
    ("stmtS", stmt_depth+3), ("stmt", stmt_depth+4), ("x", stmt_depth+5), (""),("int", stmt_depth+5), ("=", stmt_depth+5), ("*", stmt_depth+5),
    ("6", stmt_depth+6), ("7", stmt_depth+6), (";", stmt_depth+5), ("return", stmt_depth+5), ("x", stmt_depth+5),
     (";", stmt_depth+5), ("}", stmt_depth+3)];

    start.extend(stmt1.into_iter());
    start.extend(stmt2.into_iter());

    test_equality(
        start, 
        program
    );  
}

#[test]
fn decl_ass_test() {
    let program = "begin
    x: int = 5 + 5;
    x = 10 + 10;";
    let stmt_depth = 2;
    let mut start = vec![("program", 0), ("declS", 1), ("begin", 1), ("stmtS", 1)];
    let stmts = vec![
    ("stmt", stmt_depth), 
    ("x", stmt_depth+1), (":", stmt_depth+1), ("int", stmt_depth+1), 
    ("=", stmt_depth+1), ("+", stmt_depth+1), ("5", stmt_depth+2), ("5", stmt_depth+2),
    (";", stmt_depth+1),
    ("stmt", stmt_depth),
    ("x", stmt_depth+1), 
    ("=", stmt_depth+1), ("+", stmt_depth+1), ("10", stmt_depth+2), ("10", stmt_depth+2),
    (";", stmt_depth+1)
    ];
    start.extend(stmts.into_iter());
    test_equality(start, program);
}

#[test]
fn for_loop() {
    let program = 
    "begin
    sum: int = 0;
    for val in 1..=20 {
        sum = sum + val;
    }
    ";
    let mut start = vec![("program", 0), ("declS", 1), ("begin", 1), ("stmtS", 1)];
    let stmt1 = vec![("stmt", 2), ("sum", 3), (":", 3), ("int", 3), ("=", 3), ("0", 3), (";", 3)];
    let for_stmt_depth = 4;
    let for_stmt = vec![("stmt", for_stmt_depth), ("sum", for_stmt_depth + 1), 
    ("=", for_stmt_depth+ 1), ("+", for_stmt_depth+1), ("sum", for_stmt_depth+2), ("val", for_stmt_depth+2), 
    (";", for_stmt_depth+1)]; 
    let mut for_loop = vec![("stmt", 2), ("for", 3), ("val", 3), ("in", 3), 
    ("range", 3), ("1", 4), ("..=", 4), ("20", 4),
    ("{", 3), ("stmtS", 3)];
    for_loop.extend(for_stmt.into_iter());
    for_loop.push(("}", 3));

    start.extend(stmt1.into_iter());
    start.extend(for_loop.into_iter());

    test_equality(start, program);
}

#[test]
fn draw_stmt() {
    let program = "begin
    draw rectangle(|width = 4, height = 4|);
    ";
    let start = vec![("program", 0), ("declS", 1), ("begin", 1), ("stmtS", 1)];
    let draw = vec![("stmt", 2), ("draw", 3), ("SCALL",5)];
    test_equality(start, program);

}