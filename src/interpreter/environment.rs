use std::collections::HashMap;

use crate::program::{expression::Expr, program::Program, statement::Stmt};

use super::stack::Stack;

#[derive(Debug)]
pub struct IEnvironment {
    v_table: Stack<Expr>,
    f_table: Stack<(Vec<Stmt>, Vec<String>)>,
    s_table: HashMap<String, (Program, Option<Expr>)>,
    d_array: Vec<()>,
    r_value: Option<Expr>,
}

impl IEnvironment {
    pub fn new() -> Self {
        IEnvironment { v_table: Stack::new(), f_table: Stack::new(), s_table: HashMap::new(), d_array: Vec::new(), r_value: None}
    }

    pub fn vtable_push(&mut self, identifier: String, element: Expr) {
        self.v_table.push(identifier, element);
    }
    
    pub fn vtable_find(&mut self, identifier: String) -> Option<&mut Expr> {
        self.v_table.find(identifier)
    }
    
    pub fn vtable_clear(&mut self) -> Stack<Expr> {
        let vtable = self.v_table.clone();
        self.v_table.clear();
        vtable
    }

    pub fn vtable_restore(&mut self, restore: Stack<Expr>) {
        self.v_table = restore;
    }

    pub fn ftable_push(&mut self, identifier: String, statements: Vec<Stmt>, parameters: Vec<String>) {
        self.f_table.push(identifier, (statements, parameters));
    }

    pub fn ftable_find(&mut self, identifier: String) -> Option<&mut (Vec<Stmt>, Vec<String>)> {
        self.f_table.find(identifier)
    }

    pub fn push_scope(&mut self) {
        self.v_table.push_scope();
        self.f_table.push_scope();
    }

    pub fn pop_scope(&mut self) {
        self.v_table.pop_scope();
        self.f_table.pop_scope();
    }

    pub fn rvalue_set(&mut self, value: Expr) {
        self.r_value = Some(value);
    }

    pub fn rvalue_clear(&mut self) {
        self.r_value = None;
    }
    
    pub fn rvalue_get(&self) -> Option<Expr> {
        self.r_value.clone()
    }
   
    

}
