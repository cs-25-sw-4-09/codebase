use crate::program::expression::Expr;
use std::collections::HashMap;
use super::{figure::Figure, point::Point};
use crate::interpreter::value::Value;

pub struct FigureArray(Vec<Figure>);


impl FigureArray {
    pub fn get_mut_figures(&mut self) -> &mut Vec<Figure> {
        &mut self.0 
    }

}