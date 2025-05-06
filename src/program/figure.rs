use crate::program::expression::Expr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Figure {
    lines: Vec<line>,
    attributes: HashMap<String, Box::<Expr>> 
}

#[derive(Debug, PartialEq, Clone)]
pub struct line(Vec<Point>);

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: Expr, 
    y: Expr
}