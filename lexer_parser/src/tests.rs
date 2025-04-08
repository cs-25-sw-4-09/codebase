
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
    let part_program = vec![ ("program", 0), ("declS", 1), ("stmtS", 1), ("assign", 2), ("_", 3)].into_iter();
    
    let expr = vec![ ("+", 3), ("5", 4), ("*", 4), ("5", 5), ("6", 5)].into_iter();
    let nodes = part_program.clone().chain(expr).collect();

    test_equality(
        nodes, 
        "begin _ = 5 + 5 * 6;"
    );

    let expr = vec![ ("+", 3), ("*", 4), ("1", 5), ("2", 5), ("3", 4)].into_iter();
    let nodes = part_program.chain(expr).collect();
    test_equality(
        nodes, 
        "begin _ = 1 * 2 + 3;"
    );
}

#[test]
fn multi_stmt() {
    let program = "begin _ = 5; _ = 5; _ = 5; _ = 5; _ = 5; _ = 5;";
    let stmt = vec![("assign", 2), ("_", 3), ("5", 3)];
    let stmt = stmt.iter().cycle().take(stmt.len() * 6).cloned();

    test_equality(
        vec![("program", 0),("declS", 1),("stmtS", 1)].into_iter().chain(stmt).collect::<Vec<_>>(), 
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
        ("program",0), ("declS",1), ("stmtS", 1),
    ];
    let stmt1 = vec![("funcDecl", 2), 
    ("x", 3), ("params", 3), 
    ("param", 4), ("y", 5), ("int", 5), 
    ("param", 4), ("z", 5), ("int", 5), ("int",3),
    ];
    let func_stmt = vec![
        ("stmtS", 3), ("return", 4), ("+", 5), ("y", 6), ("z", 6)
    ];
    let stmt2 = vec![
        ("varDecl", 2), ("a", 3), ("int", 3),
        ("FCall", 3),
        ("x", 4),
        ("args", 4), ("5", 5), ("6", 5)
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
        ("varDecl", stmt_l), ("x", stmt_l + 1), ("bool", stmt_l + 1), ("true", stmt_l + 1),
        ("assign", stmt_l), ("x", stmt_l + 1), ("!", stmt_l + 1), ("x", stmt_l + 2),
    ];
    let mut nodes = vec![
        ("program", 0), ("declS", 1), ("stmtS", 1)
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

    let mut start = vec![("program", 0), ("declS",1), ("stmtS", 1)];
    let stmt1 = vec![ 
        ("fork", 2), ("forkExpr", 3), (">",4),
        ("5", 5), ("4", 5), ("stmtS", 4), ("return", 5),
        ("10", 6)];
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
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    let stmt_depth = 1;
    let stmt1 = vec![("fork", stmt_depth+1), 
    ("forkExpr", stmt_depth+2), 
    ("==", stmt_depth+3), 
    ("5", stmt_depth+4), ("5", stmt_depth+4),
    ("stmtS", stmt_depth+3), ("return", stmt_depth+4), ("+", stmt_depth+5), ("5", stmt_depth+6),
     ("5", stmt_depth+6)
    ];

    let stmt2 = vec![("forkExpr", stmt_depth+2), (">", stmt_depth+3), 
    ("5", stmt_depth+4), ("3", stmt_depth+4),
    ("stmtS", stmt_depth+3), ("varDecl", stmt_depth+4), ("x", stmt_depth+5), ("int", stmt_depth+5), ("*", stmt_depth+5),
    ("6", stmt_depth+6), ("7", stmt_depth+6), ("return", stmt_depth+4), ("x", stmt_depth+5),
     ];

    start.extend(stmt1.into_iter());
    start.extend(stmt2.into_iter());

    test_equality(
        start, 
        program
    );  
}

#[test]
fn otherwise_fork(){
    let program = 
    "begin
    fork {
    (10 == 10) -> {return 1;}
    (otherwise) -> {return 0;}
    }
    ";
    let stmt_depth = 1;
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    let stmt1 = vec![("fork", stmt_depth+1), ("forkExpr", stmt_depth+2), 
    ("==", stmt_depth+3), ("10", stmt_depth+4), ("10", stmt_depth+4), ("stmtS", stmt_depth+3),
    ("return", stmt_depth+4), ("1", stmt_depth+5)];
    let stmt2 = vec![("otherwise", stmt_depth+2), ("stmtS", stmt_depth+3), ("return", stmt_depth+4), ("0", stmt_depth+5)];

    start.extend(stmt1.into_iter());
    start.extend(stmt2.into_iter());
    
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
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    let stmt1 = vec![("varDecl", 2), ("sum", 3), ("int", 3), ("0", 3)];
    let for_stmt_depth = 4;
    
    let mut for_loop = vec![("for", 2), ("val", 3),
    ("range", 3), ("1", 4), ("..=", 4), ("20", 4),
   ("stmtS", 3)];
    let for_stmt = vec![("assign", for_stmt_depth), ("sum", for_stmt_depth + 1), 
    ("+", for_stmt_depth+1), ("sum", for_stmt_depth+2), ("val", for_stmt_depth+2), 
    ]; 
    for_loop.extend(for_stmt.into_iter());

    start.extend(stmt1.into_iter());
    start.extend(for_loop.into_iter());

    test_equality(start, program);
}

#[test]
fn draw_stmt() {
    let program = "begin
    draw rectangle(|width = 4, height = 4|);
    ";
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    let draw = vec![ ("draw", 2), ("SCall",3), ("rectangle", 4), ("attrS", 4), ("attr", 5), ("width", 6),
    ("4", 6), ("attr",  5), ("height", 6), ("4", 6)];
    start.extend(draw.iter());
    test_equality(start, program);


}

#[test]
fn precedence_int() {
    let program = "begin
        _ = (5 + 5) * 6 / 7 % 20 + 7 - 8;
        _ = 6 * (5 % 20 + 10) / 7;
    ";
    
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    let assign_level = 2;
    let assign1 = vec![("assign", assign_level), ("_", assign_level + 1),
        ("-", assign_level + 1), ("+", assign_level + 2), ("%", assign_level + 3), ("/", assign_level+4),
        ("*", assign_level + 5),
        ("+", assign_level + 6), ("5", assign_level+7), ("5", assign_level+7),
        ("6", assign_level + 6),
        ("7", assign_level + 5),
        ("20", assign_level + 4),
        ("7", assign_level + 3),
        ("8", assign_level + 2)
    ];
    let assign2 = vec![("assign", assign_level), ("_", assign_level + 1),
    ("/", assign_level+1), 
    ("*", assign_level + 2), ("6", assign_level + 3),
    ("+", assign_level + 3), ("%", assign_level + 4), ("5", assign_level + 5), ("20", assign_level + 5),
    ("10", assign_level + 4),
    ("7", assign_level + 2)
    ];

    start.extend(assign1.iter());
    start.extend(assign2.iter());
    
    test_equality(start, program);

}

#[test]
fn precedence_bool() {
    let program = 
    "begin
    x: bool = (true || false) && (5 < 6) 
    || (((5 + 9) == 0) && (true)) || (false) || (4 <= 4) && (5 > 1) && (4 >= 7);
    ";

    let bool_level = 2;
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    let stmt1 = vec![("varDecl", bool_level), ("x", bool_level+1), ("bool", bool_level+1),
    ("||", bool_level+1), ("||", bool_level+2), ("||", bool_level+3), ("&&", bool_level+4),
    ("||",  bool_level+5), ("true",  bool_level+6), ("false",  bool_level+6), ("<", bool_level+5), ("5", bool_level+6), ("6", bool_level+6),
    ("&&", bool_level+4), ("==", bool_level+5), ("+", bool_level+6),("5", bool_level+7), ("9", bool_level+7), ("0", bool_level+6), ("true", bool_level+5),
    ("false", bool_level+3), ("&&", bool_level+2),("&&", bool_level+3), ("<=", bool_level+4),("4", bool_level+5),("4", bool_level+5),
    (">", bool_level+4),("5", bool_level+5),("1", bool_level+5),(">=", bool_level+3),("4", bool_level+4),("7", bool_level+4)];
    start.extend(stmt1.into_iter());
    test_equality(start, program);
}

#[test]
fn point_decl(){
    let program = "
    begin
    x: point = (1 + 3, 4 + 5);
    ";
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    start.extend(
        vec![
            ("varDecl", 2), ("x", 3), ("point", 3), ("point", 3), ("+", 4), ("1", 5), ("3", 5),
            ("+", 4), ("4", 5), ("5", 5)
        ].into_iter()
    );
    test_equality(start, program);
}

#[test]
fn color_decl(){
    let program = "
    begin
    x: color = (1, 2, 3, 4);
    ";
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    start.extend(
        vec![
            ("varDecl", 2), ("x", 3), ("color", 3), ("color", 3), ("1", 4), ("2", 4), ("3", 4),
            ("4", 4),
        ].into_iter()
    );
    test_equality(start, program);
}

#[test]
fn straight_path_decl(){
    let straight_path = "begin
    x: path = p1--(2,4)--*;
    ";
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    start.extend(
        vec![
            ("varDecl", 2), ("x", 3), ("path", 3), ("path", 3), ("p1", 4), ("--", 4), ("point", 4), ("2", 5), ("4", 5),
            ("--", 4), ("*", 4)
        ].into_iter()
    );
    test_equality(start, straight_path);
}

#[test]
fn curved_path_decl(){
    let curved_path = "
    begin
    x: path = p1~~(2,4)~~p3;
    ";
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    start.extend(
        vec![
            ("varDecl", 2), ("x", 3), ("path", 3), ("path", 3), ("p1", 4), ("~~", 4), ("point", 4), ("2", 5), ("4", 5),
            ("~~", 4), ("p3", 4)
        ].into_iter()
    );
    test_equality(start, curved_path);
}

#[test]
fn mixed_path_decl(){
    let mixed_path = "
    begin
    x: path = p1--(2,4)~~p3;
    ";
    let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
    start.extend(
        vec![
            ("varDecl", 2), ("x", 3), ("path", 3), ("path", 3), ("p1", 4), ("--", 4), ("point", 4), ("2", 5), ("4", 5),
            ("~~", 4), ("p3", 4)
        ].into_iter()
    );
    test_equality(start, mixed_path);
}

#[test]
fn array_test(){
let program = 
"begin
x: int[] = [5, 3, 5, 7];
y: int[][] = [[1,2],[3,4]];
";
let array_depth = 2;
let mut start = vec![("program", 0), ("declS", 1), ("stmtS", 1)];
let stmt1 = vec![("varDecl", array_depth), ("x", array_depth+1), ("int", array_depth+1), ("",0)];
    test_equality(start, program);
}
