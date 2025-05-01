use crate::program::{expression::Expr, statement::Stmt, r#type::Type};

use super::stack::Stack;

#[derive(Debug)]
pub struct IEnvironment {
    v_table: Stack<Expr>,
    f_table: Stack<(Vec<Stmt>, Vec<String>)>,
    //  s_table: HashMap<String, (Attributes, Program)>,
    // d_array: Vec<()>,
    r_value: Expr,
}

impl IEnvironment {
    pub fn new() -> Self {
        IEnvironment { v_table: Stack::new(), f_table: Stack::new(), r_value: Expr::Integer(0)}
    }

    pub fn vtable_push(&mut self, identifier: String, element: Expr) {
        self.v_table.push(identifier, element);
    }

    pub fn vtable_pop(&mut self) {
        self.v_table.pop();
    }

    pub fn ftable_push(&mut self, identifier: String, statements: Vec<Stmt>, parameters: Vec<String>) {
        self.f_table.push(identifier, (statements, parameters));
    }

    pub fn ftable_pop(&mut self) {
        self.f_table.pop();
    }

    pub fn ftable_find(&mut self, identifier: String) -> Option<(Vec<Stmt>, Vec<String>)> {
        self.f_table.find(identifier)
    }

    pub fn push_scope(&mut self) {
        self.v_table.push_vscope();
        self.f_table.push_fscope();
    }

    pub fn rvalue_set(&mut self, value: Expr) {
        self.r_value = value;
    }
    
    pub fn rvalue_get(&self) -> Expr {
        self.r_value.clone()
    }
    

}
