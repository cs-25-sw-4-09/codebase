use crate::program::expression::Expr;
use std::collections::HashMap;
use super::{figure::Figure, point::Point};
use crate::interpreter::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct FigureArray(Vec<Figure>);


impl FigureArray {
    pub fn get_mut_figures(&mut self) -> &mut Vec<Figure> {
        &mut self.0 
    }

    pub fn get_figures(&self) -> &Vec<Figure> {
        &self.0
    }

}


impl From<Vec<Figure>> for FigureArray {
    fn from(value: Vec<Figure>) -> Self {
        Self(value)
    }
}

impl FigureArray {
    pub fn get_top_left(&self) -> Point {
        todo!()
    }
}