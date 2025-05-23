use super::super::{
    grammar::cfg, utils::{
        tree_builder::TreeBuilderStr, tree_converter::stringify_tree
    }, valid_programs::{get_programs, get_programs2}
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

/* Example Program parsing */
#[test]
fn example_program_parsing_expr_value() {
    //A program is succesfully parsed if get_root() does not panic.
    get_programs2()
    .into_iter()
    .for_each(|el| {
        let _ = cfg::parse_str(el).get_ast().get_root();
    });
}

/* Expressions */
#[test]
fn test_multi_and_add_precedence() {
    let part_program = vec![ ("Program", 0), ("DeclS", 1), ("StmtS", 1), ("Assign", 2), ("_", 3)].into_iter();

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
    let stmt = vec![("Assign", 2), ("_", 3), ("5", 3)];
    let stmt = stmt.iter().cycle().take(stmt.len() * 6).cloned();

    test_equality(
        vec![("Program", 0),("DeclS", 1),("StmtS", 1)].into_iter().chain(stmt).collect::<Vec<_>>(),
        program
    );
}

#[test]
fn function_declaration_and_call() {
    let program =
    "begin
    x(y: int, z: int): int -> {
        return y+z;
    }
    a: int = x(5, 6);
    ";
    let mut start = vec![
        ("Program",0), ("DeclS",1), ("StmtS", 1),
    ];
    let stmt1 = vec![("FuncDecl", 2),
    ("x", 3), ("Params", 3),
    ("Param", 4), ("y", 5), ("int", 5),
    ("Param", 4), ("z", 5), ("int", 5), ("int",3),
    ];
    let func_stmt = vec![
        ("StmtS", 3), ("Return", 4), ("+", 5), ("y", 6), ("z", 6)
    ];
    let stmt2 = vec![
        ("VarDecl", 2), ("a", 3), ("int", 3),
        ("FCall", 3),
        ("x", 4),
        ("Args", 4), ("5", 5), ("6", 5)
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
        ("VarDecl", stmt_l), ("x", stmt_l + 1), ("bool", stmt_l + 1), ("true", stmt_l + 1),
        ("Assign", stmt_l), ("x", stmt_l + 1), ("!", stmt_l + 1), ("x", stmt_l + 2),
    ];
    let mut nodes = vec![
        ("Program", 0), ("DeclS", 1), ("StmtS", 1)
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

    let mut start = vec![("Program", 0), ("DeclS",1), ("StmtS", 1)];
    let stmt1 = vec![
        ("Fork", 2), ("ForkCase", 3), (">",4),
        ("5", 5), ("4", 5), ("StmtS", 4), ("Return", 5),
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
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt_depth = 1;
    let stmt1 = vec![("Fork", stmt_depth+1),
    ("ForkCase", stmt_depth+2),
    ("==", stmt_depth+3),
    ("5", stmt_depth+4), ("5", stmt_depth+4),
    ("StmtS", stmt_depth+3), ("Return", stmt_depth+4), ("+", stmt_depth+5), ("5", stmt_depth+6),
     ("5", stmt_depth+6)
    ];

    let stmt2 = vec![("ForkCase", stmt_depth+2), (">", stmt_depth+3),
    ("5", stmt_depth+4), ("3", stmt_depth+4),
    ("StmtS", stmt_depth+3), ("VarDecl", stmt_depth+4), ("x", stmt_depth+5), ("int", stmt_depth+5), ("*", stmt_depth+5),
    ("6", stmt_depth+6), ("7", stmt_depth+6), ("Return", stmt_depth+4), ("x", stmt_depth+5),
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
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("Fork", stmt_depth+1), ("ForkCase", stmt_depth+2),
    ("==", stmt_depth+3), ("10", stmt_depth+4), ("10", stmt_depth+4), ("StmtS", stmt_depth+3),
    ("Return", stmt_depth+4), ("1", stmt_depth+5)];
    let stmt2 = vec![("Otherwise", stmt_depth+2), ("StmtS", stmt_depth+3), ("Return", stmt_depth+4), ("0", stmt_depth+5)];

    start.extend(stmt1.into_iter());
    start.extend(stmt2.into_iter());

    test_equality(start, program);
}

#[test]
fn for_loop() {
    let program =
    "begin
    sum: int = 0;
    for val in 1 to 20 {
        sum = sum + val;
    }
    ";
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("VarDecl", 2), ("sum", 3), ("int", 3), ("0", 3)];
    let for_stmt_depth = 4;

    let mut for_loop = vec![("For", 2), ("val", 3), ("1", 3), ("20", 3),
   ("StmtS", 3)];
    let for_stmt = vec![("Assign", for_stmt_depth), ("sum", for_stmt_depth + 1),
    ("+", for_stmt_depth+1), ("sum", for_stmt_depth+2), ("val", for_stmt_depth+2),
    ];
    for_loop.extend(for_stmt.into_iter());

    start.extend(stmt1.into_iter());
    start.extend(for_loop.into_iter());

    test_equality(start, program);
}

#[test]
fn draw_stmt_and_scall() {
    let program = "begin
    draw rectangle(|width = 4, height = 4|);
    ";
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let draw = vec![ ("Draw", 2), ("SCall",3), ("rectangle", 4), ("AttrS", 4), ("Attr", 5), ("width", 6),
    ("4", 6), ("Attr",  5), ("height", 6), ("4", 6)];
    start.extend(draw.iter());
    test_equality(start, program);


}

#[test]
fn precedence_int() {
    let program = "begin
        _ = (5 + 5) * 6 / 7 % 20 + 7 - 8;
        _ = 6 * (5 % 20 + 10) / 7;
    ";

    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let assign_level = 2;
    let assign1 = vec![("Assign", assign_level), ("_", assign_level + 1),
        ("-", assign_level + 1), ("+", assign_level + 2), ("%", assign_level + 3), ("/", assign_level+4),
        ("*", assign_level + 5),
        ("+", assign_level + 6), ("5", assign_level+7), ("5", assign_level+7),
        ("6", assign_level + 6),
        ("7", assign_level + 5),
        ("20", assign_level + 4),
        ("7", assign_level + 3),
        ("8", assign_level + 2)
    ];
    let assign2 = vec![("Assign", assign_level), ("_", assign_level + 1),
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
    x = 5 != 4;
    ";

    let bool_level = 2;
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("VarDecl", bool_level), ("x", bool_level+1), ("bool", bool_level+1),
    ("||", bool_level+1), ("||", bool_level+2), ("||", bool_level+3), ("&&", bool_level+4),
    ("||",  bool_level+5), ("true",  bool_level+6), ("false",  bool_level+6), ("<", bool_level+5), ("5", bool_level+6), ("6", bool_level+6),
    ("&&", bool_level+4), ("==", bool_level+5), ("+", bool_level+6),("5", bool_level+7), ("9", bool_level+7), ("0", bool_level+6), ("true", bool_level+5),
    ("false", bool_level+3), ("&&", bool_level+2),("&&", bool_level+3), ("<=", bool_level+4),("4", bool_level+5),("4", bool_level+5),
    (">", bool_level+4),("5", bool_level+5),("1", bool_level+5),(">=", bool_level+3),("4", bool_level+4),("7", bool_level+4)];
    let stmt2 = vec![
        ("Assign", bool_level),
        ("x", bool_level + 1),
        ("!=", bool_level + 1),
        ("5", bool_level +2),
        ("4", bool_level + 2)
    ];

    start.extend(stmt1.into_iter().chain(stmt2.into_iter()));
    test_equality(start, program);
}

#[test]
fn point_decl(){
    let program = "
    begin
    x: point = (1 + 3, 4 + 5);
    ";
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    start.extend(
        vec![
            ("VarDecl", 2), ("x", 3), ("point", 3), ("Point", 3), ("+", 4), ("1", 5), ("3", 5),
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
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    start.extend(
        vec![
            ("VarDecl", 2), ("x", 3), ("color", 3), ("Color", 3), ("1", 4), ("2", 4), ("3", 4),
            ("4", 4),
        ].into_iter()
    );
    test_equality(start, program);
}

#[test]
fn straight_polygon_decl(){
    let straight_path = "begin
    x: polygon = p1--(2,4)--*;
    ";
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    start.extend(
        vec![
            ("VarDecl", 2), ("x", 3), ("polygon", 3), ("--*", 3), ("--", 4), ("p1", 5), ("Point", 5), ("2", 6), ("4", 6)
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
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    start.extend(
        vec![
            ("VarDecl", 2), ("x", 3), ("path", 3), ("~~", 3), ("~~",4), ("p1", 5), ("Point", 5), ("2", 6), ("4", 6),
            ("p3", 4)
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
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    start.extend(
        vec![
            ("VarDecl", 2), ("x", 3), ("path", 3), ("~~", 3), ("--",4), ("p1", 5), ("Point", 5), ("2", 6), ("4", 6),
            ("p3", 4)
        ].into_iter()
    );
    test_equality(start, mixed_path);
}

#[test]
fn array_test(){
    let program =
    "begin
    x: int[] = [5, 3, 5, 7];
    z: int[] = [];
    ";
    let array_depth = 2;
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("VarDecl", array_depth), ("x", array_depth+1), ("int[]", array_depth+1), ("Array", array_depth+1),
    ("5", array_depth+2), ("3", array_depth+2), ("5", array_depth+2), ("7", array_depth+2),];

    let stmt3 = vec![("VarDecl" , array_depth), ("z", array_depth+1), ("int[]", array_depth+1), ("Array", array_depth+1)];
    start.extend(stmt1.into_iter().chain(stmt3.into_iter()));
    test_equality(start, program);
}

#[test]
fn comment_stmt() {
    let program = "begin
    // draw rectangle
    draw rectangle(|width = 4, height = 4|);
    /*This comment should be ignored*/
    ";
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let draw = vec![ ("Draw", 2), ("SCall",3), ("rectangle", 4), ("AttrS", 4), ("Attr", 5), ("width", 6),
    ("4", 6), ("Attr",  5), ("height", 6), ("4", 6)];
    start.extend(draw.iter());
    test_equality(start, program);
}


#[test]
fn manipulation_place(){
    let program =
    "begin
    x = place rec left x;
    x = place rec right (10,10) offset 5;
    ";
    let manipulation_depth = 2;
    let mut start  = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("Assign", manipulation_depth),
    ("x", manipulation_depth+1), ("Manipulation", manipulation_depth+1),
    ("Place", manipulation_depth+2), ("rec", manipulation_depth+3),
    ("left", manipulation_depth+3), ("x", manipulation_depth+3)];

    let stmt2 = vec![
        ("Assign", manipulation_depth),
        ("x", manipulation_depth+1), ("Manipulation", manipulation_depth+1),
        ("Place", manipulation_depth+2), ("rec", manipulation_depth+3),
        ("right", manipulation_depth+3), ("point", manipulation_depth+3),
        ("10", manipulation_depth+4), ("10", manipulation_depth+4),  ("5", manipulation_depth+3),
    ];

    println!("{:?}", stmt1.into_iter().chain(stmt2.into_iter()));

    //start.extend(stmt1.into_iter().chain(stmt2.into_iter()));
    //test_equality(start, program);
}

#[test]
fn manipulation_place1(){
    let program =
    "begin
    x = place rec left x;
    ";
    let manipulation_depth = 2;
    let mut start  = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("Assign", manipulation_depth),
    ("x", manipulation_depth+1), ("Manipulation", manipulation_depth+1),
    ("Place", manipulation_depth+2), ("rec", manipulation_depth+3),
    ("left", manipulation_depth+3), ("x", manipulation_depth+3)];

    start.extend(stmt1.into_iter());
    test_equality(start, program);
}

#[test]
fn manipulation_place2(){
    let program =
    "begin
    x = place rec right (10,10) offset 5;
    ";
    let manipulation_depth = 2;
    let mut start  = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];

    let stmt2 = vec![
        ("Assign", manipulation_depth),
        ("x", manipulation_depth+1), ("Manipulation", manipulation_depth+1),
        ("Place", manipulation_depth+2), ("rec", manipulation_depth+3),
        ("right", manipulation_depth+3), ("Point", manipulation_depth+3),
        ("10", manipulation_depth+4), ("10", manipulation_depth+4),  ("5", manipulation_depth+3),
    ];

    start.extend(stmt2.into_iter());
    test_equality(start, program);
}

#[test]
fn manipulation_scale(){
    let program =
    "begin
    x = scale rec by 10;
    ";
    let manipulation_depth = 2;
    let mut start  = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("Assign", manipulation_depth),
    ("x", manipulation_depth+1), ("Manipulation", manipulation_depth+1),
    ("Scale", manipulation_depth+2), ("rec", manipulation_depth+3), ("10", manipulation_depth+3)
    ];
    start.extend(stmt1.into_iter());

    test_equality(start, program);
}

#[test]
fn manipulation_rotate(){
    let program =
    "begin
    x = rotate rec by 10;
    ";
    let manipulation_depth = 2;
    let mut start  = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("Assign", manipulation_depth),
    ("x", manipulation_depth+1), ("Manipulation", manipulation_depth+1),
    ("Rotate", manipulation_depth+2), ("rec", manipulation_depth+3), ("10", manipulation_depth+3)
    ];

    start.extend(stmt1.into_iter());

    test_equality(start, program);
}

#[test]
fn properties() {
    let program = "begin
        p: point = (1,2);
        x: int = p.x;
    ";
    let start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt_depth = 2;
    let stmt1 = vec![("VarDecl", 2), ("p", 3), ("point", 3), ("Point", 3), ("1", 4), ("2", 4)];
    let stmt2 = vec![("VarDecl", stmt_depth), ("x", stmt_depth + 1), ("int", stmt_depth + 1),
        ("Member", stmt_depth+ 1),
        ("p", stmt_depth+2), ("x", stmt_depth+2)
    ];

    let nodes = start.into_iter().chain(stmt1.into_iter()).chain(stmt2.into_iter()).collect();
    test_equality(nodes, program);
}

#[test]
fn indexing(){
    let program =
    "begin
    x : int[] = [1,2,3,4];
    a = x[1];
    ";
    let indexing_depth = 2;
    let mut start = vec![("Program", 0), ("DeclS", 1), ("StmtS", 1)];
    let stmt1 = vec![("VarDecl", indexing_depth), ("x", indexing_depth+1), ("int[]", indexing_depth+1), ("Array", indexing_depth+1), ("1", indexing_depth+2), ("2", indexing_depth+2), ("3", indexing_depth+2),
    ("4", indexing_depth+2)];
    let stmt2 = vec![("Assign", indexing_depth), ("a", indexing_depth+1), ("ArrayIdx", indexing_depth+1), ("x", indexing_depth+2), ("1", indexing_depth+2)];

    start.extend(stmt1.into_iter().chain(stmt2.into_iter()));

    test_equality(start, program);
}
