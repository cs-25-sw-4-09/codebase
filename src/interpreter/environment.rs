use std::collections::HashMap;

use super::data_types::{figure::Figure, figurearray::FigureArray};
use super::stack::Stack;
use super::value::Value;
use crate::program::{program::Program, statement::Stmt};

#[derive(Debug)]
pub struct IEnvironment {
    v_table: Stack<Value>,
    f_table: Stack<(Vec<Stmt>, Vec<String>)>,
    s_table: HashMap<String, Program>,
    d_array: FigureArray,
    r_value: Option<Value>,
}

impl IEnvironment {
    pub fn new() -> Self {
        IEnvironment {
            v_table: Stack::new(),
            f_table: Stack::new(),
            s_table: HashMap::new(),
            d_array: FigureArray::new(),
            r_value: None,
        }
    }

    pub fn vtable_push(&mut self, identifier: String, element: Value) {
        self.v_table.push(identifier, element);
    }

    pub fn vtable_find(&mut self, identifier: String) -> Option<&mut Value> {
        self.v_table.find(identifier)
    }

    pub fn vtable_clear(&mut self) -> Stack<Value> {
        let vtable = self.v_table.clone();
        self.v_table.clear();
        vtable
    }

    pub fn vtable_restore(&mut self, restore: Stack<Value>) {
        self.v_table = restore;
    }

    pub fn ftable_push(
        &mut self,
        identifier: String,
        statements: Vec<Stmt>,
        parameters: Vec<String>,
    ) {
        self.f_table.push(identifier, (statements, parameters));
    }

    pub fn ftable_find(&mut self, identifier: String) -> Option<&mut (Vec<Stmt>, Vec<String>)> {
        self.f_table.find(identifier)
    }

    pub fn stable_push(&mut self, identifier: String, program: Program) {
        self.s_table.insert(identifier, program);
    }

    pub fn stable_find(&mut self, identifier: String) -> Option<&mut Program> {
        self.s_table.get_mut(&identifier)
    }

    pub fn darray_get(&self) -> &FigureArray {
        &self.d_array
    }

    pub fn darray_push(&mut self, shape: FigureArray){
        self.d_array.extend(shape);


    pub fn push_scope(&mut self) {
        self.v_table.push_scope();
        self.f_table.push_scope();
    }

    pub fn pop_scope(&mut self) {
        self.v_table.pop_scope();
        self.f_table.pop_scope();
    }

    pub fn rvalue_set(&mut self, value: Value) {
        self.r_value = Some(value);
    }

    pub fn rvalue_clear(&mut self) {
        self.r_value = None;
    }

    pub fn rvalue_get(&self) -> Option<Value> {
        self.r_value.clone()
    }


}
