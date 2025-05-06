use crate::program::expression::Expr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Figure {
    lines: Vec<Line>,
    attributes: HashMap<String, Box::<Expr>> 
}

impl Figure {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            attributes: HashMap::new()
        }
    }

    pub fn push_line(&mut self, ps: Vec<Point>) {
        self.lines.push(Line::from(ps));
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line(Vec<Point>);
/*impl From<(Point, Point)> for Line {
    fn from(value: (Point, Point)) -> Self {
        Self(vec![value.0, value.1])
    }
}*/
impl From<Vec<Point>> for Line {
    fn from(value: Vec<Point>) -> Self { Self(value) }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: Box::<Expr>, 
    y: Box::<Expr>
}

impl From<&Box<Expr>> for Point {
    fn from(value: &Box<Expr>) -> Self {
        match value.as_ref() {
            Expr::Point(expr, expr1) => Self { x: expr.clone(), y: expr1.clone() }, 
            _ => todo!(),
        } 
    }
}