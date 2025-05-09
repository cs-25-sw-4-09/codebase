use std::error::Error;
use crate::{
    interpreter::{
        data_types::{figure::Figure, figurearray::FigureArray }, errors::{MaxCanNotBeFound, MinCanNotBeFound}
    }, 
    program::{expression::Expr, operators::pathoperator::PathOperator}};
use crate::interpreter::value::Value;


pub fn scale(mut shape: FigureArray, factor: Value) -> Result<(),Box<dyn Error>> { 
    let origin_x = shape.get_mut_lines().iter().map(|l| l.get_max_x()).max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).ok_or_else(||Err(MinCanNotBeFound))?;
    let origin_y = shape.get_mut_lines().iter().map(|l| l.get_min_y()).min_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).ok_or_else(||Err(MaxCanNotBeFound))?;

    for line in shape.get_mut_lines() { 

    }

    Ok(())


    }
  


