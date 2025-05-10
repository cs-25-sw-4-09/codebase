use crate::{interpreter::errors, program::expression::Expr};
use std::{collections::HashMap, error::Error};
use super::{figure::Figure, point::Point};
use crate::interpreter::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct FigureArray(Vec<Figure>);


impl FigureArray {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get_mut_figures(&mut self) -> &mut Vec<Figure> {
        &mut self.0 
    }

    pub fn get_figures(&self) -> &Vec<Figure> {
        &self.0
    }

    pub fn extend(&mut self, shape: FigureArray){
        self.0.extend(shape.0);
    }

    //todo: optimize
    pub fn height(&self) -> Value {
        Value::Float(
            self.0.iter().map(|fig| fig.get_max_y())
            .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0f64) -  
            self.0.iter().map(|fig| fig.get_min_y())
            .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0f64) 
        )
    }
    //todo: optimize
    pub fn width(&self) -> Value {
        Value::Float(
            self.0.iter().map(|fig| fig.get_max_x())
            .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0f64) -  
            self.0.iter().map(|fig| fig.get_min_x())
            .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0f64) 
        )

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

    pub fn push_figures(&mut self, shape: FigureArray) {
        shape.0.into_iter().for_each(|fig| self.0.push(fig));
    }
}