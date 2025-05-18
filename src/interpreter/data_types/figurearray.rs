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

    pub fn extend(&mut self, shape: FigureArray) {
        self.0.extend(shape.0);
    }

    pub fn max_x(&self) -> Value {
        self.0
            .iter()
            .map(|fig| fig.get_max_x())
            .max()
            .unwrap_or(0.into())
    }

    pub fn min_x(&self) -> Value {
        self.0
            .iter()
            .map(|fig| fig.get_min_x())
            .min()
            .unwrap_or(0.into())
    }
    pub fn max_y(&self) -> Value {
        self.0
            .iter()
            .map(|fig| fig.get_max_y())
            .max()
            .unwrap_or(0.into())
    }
    pub fn min_y(&self) -> Value {
        self.0
            .iter()
            .map(|fig| fig.get_min_y())
            .min()
            .unwrap_or(0.into())
    }

    pub fn flip_y(&mut self) {
        self.0.iter_mut().for_each(|fig| {
            fig.get_mut_lines().iter_mut().for_each(|l| {
                l.get_mut_points()
                    .iter_mut()
                    .for_each(|p| p.set_y(-p.get_y()))
            })
        });
    }

    //todo: optimize
    pub fn height(&self) -> Value {
        self.max_y() - self.min_y()
    }
    //todo: optimize
    pub fn width(&self) -> Value {
        self.max_x() - self.min_x()
    }

    pub fn get_top_left(&self) -> Point {
        (self.min_x(), self.max_y()).into()
    }

    pub fn get_center(&self) -> Point {
        (
            self.min_x() + &self.width() / &2.0.into(),
            self.min_y() + &self.height() / &2.0.into(),
        )
            .into()
    }
}

impl From<Vec<Figure>> for FigureArray {
    fn from(value: Vec<Figure>) -> Self {
        Self(value)
    }
}
