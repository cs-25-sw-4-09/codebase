use crate::{
    interpreter::{
        environment::IEnvironment, InterpretE, 
        data_types::{figure::{Figure, Line}, point::Point}
    }, 
    program::{expression::Expr, operators::pathoperator::PathOperator}};
use crate::interpreter::value::Value;


/*pub fn scale(shape: Figure, factor: Value) { 
    let origin_x = shape.get_min_x() as f64;
    let origin_y = shape.get_max_y() as f64;

    for line in shape() {
        for point in line {

        }
    }
    let lefttop: Vec<i64> = vec![shape.get_max_x(), shape.get_min_y()];
    shape.get_lines().iter().map(|l| l.get_points() - lefttop[1])
}
*/
