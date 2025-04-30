use std::{collections::HashMap, vec};

use crate::program::{expression::Expr, statement::Stmt};

use super::stack::Stack;

#[derive(Debug)]
pub struct IEnvironment {
    v_table: Stack<Expr>,
    //    f_table: Stack<()>,
    //  s_table: HashMap<String, (Attributes, Program)>,
    // d_array: Vec<()>,
    // r_value: Expr,
}

impl IEnvironment {
    pub fn new() -> Self {
        IEnvironment { v_table: Stack::new() }
    }
}
