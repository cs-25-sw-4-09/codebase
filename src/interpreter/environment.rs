use crate::program::expression::Expr;

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

    pub fn vtable_push(&mut self, identifier: String, element: Expr) {
        self.v_table.push(identifier, element);
    }

    pub fn vtable_pop(&mut self) {
        self.v_table.pop();
        
    }

}
