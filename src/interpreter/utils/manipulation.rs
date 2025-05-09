use crate::interpreter::value::Value;
use crate::{
    interpreter::{
        data_types::{figure::Figure, figurearray::FigureArray},
        errors,
    },
    program::{expression::Expr, operators::pathoperator::PathOperator},
};
use std::error::Error;

pub fn scale(mut shape: FigureArray, factor: Value) -> Result<(), Box<dyn Error>> {
    let origin_x = shape
        .get_mut_lines()
        .iter()
        .map(|l| l.get_max_x())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or_else(|| Box::new(errors::MinCanNotBeFound))?;
    let origin_y = shape
        .get_mut_lines()
        .iter()
        .map(|l| l.get_min_y())
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or_else(|| Box::new(errors::MaxCanNotBeFound))?;

    for line in shape.get_mut_lines() {}

    Ok(())
}
