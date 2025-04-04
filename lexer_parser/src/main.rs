

use lexer_parser::{
    grammar::cfg,
    utils::{tree_converter::{
        stringify_tree,
        print
    }, tree_builder::TreeBuilderStr},
    valid_programs::get_programs
};


fn main() {
    //!Semicolon og () i fork er ændret
    get_programs().into_iter().take(2).for_each(|el| { let _ = cfg::parse_str(el).get_ast().get_root(); });
    let mut start = vec![
        ("program",0), ("decl",1), ("begin", 1), ("stmtS", 1),
    ];    
    let mut stmt1 = vec![("stmt", 2), 
    ("x", 3), ("(", 3), ("params", 3),
    ("param", 4),
    ("y", 5), (":", 5), ("int", 5), 
    (",", 4), ("param", 4),
    ("z", 5), (":", 5), ("int", 5),
    (")",3), (":",3), ("int",3), ("->",3)
    ];

    let func_stmt = vec![
        ("{", 3),("stmt", 3), ("return", 4), ("+", 4), ("y", 5), ("z", 5), (";", 4), ("}", 3)
    ];
    stmt1.extend(func_stmt.iter());

    let stmt2 = vec![
        ("stmt", 2), ("a", 3), (":", 3), ("int", 3), ("=", 3), 
        ("FCall", 3),
        ("x", 4), ("(", 4), 
        ("args", 4), ("arg", 5), ("5", 6),("arg", 5), ("6", 6),
        (")", 4), (";", 3)
    ];

    start.extend(stmt1.iter());
    start.extend(stmt2.iter());

    let program = 
    "begin
    x(y: int, z: int): int -> {
        return y+z;
    } 
    a: int = x(5, 6);
    ";
    println!("{}\n\n", stringify_tree(cfg::parse_str(program).get_ast().get_root()));

   
    println!("{}", TreeBuilderStr::new().multi_add(&start).build());

}

