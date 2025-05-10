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

    pub fn max_x(&self) -> f64 {
        self.0.iter().map(|fig| fig.get_max_x())
        .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0f64)
    }

    pub fn min_x(&self) -> f64 {
        self.0.iter().map(|fig| fig.get_min_x())
            .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0f64) 
    }
    pub fn max_y(&self) -> f64 {
        self.0.iter().map(|fig| fig.get_max_y())
            .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0f64)
    }
    pub fn min_y(&self) -> f64 {
        self.0.iter().map(|fig| fig.get_min_y())
        .max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0f64)
    }

    //todo: optimize
    pub fn height(&self) -> Value {
        Value::Float(self.max_y() -  self.min_y())
    }
    //todo: optimize
    pub fn width(&self) -> Value {
        Value::Float(self.max_x() -  self.min_x())
    }

    pub fn get_top_left(&self) -> Point {
        (Value::Float(self.min_x()), Value::Float(self.max_y())).into()
    }

}


impl From<Vec<Figure>> for FigureArray {
    fn from(value: Vec<Figure>) -> Self {
        Self(value)
    }
}
