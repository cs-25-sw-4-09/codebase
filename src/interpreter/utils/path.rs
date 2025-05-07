
use crate::{
    interpreter::{
        environment::IEnvironment, InterpretE, 
        data_types::{figure::{Figure, Line}, point::Point}
    }, 
    program::{expression::Expr, operators::pathoperator::PathOperator}};
use crate::interpreter::value::Value;

#[derive(PartialEq)]
enum ToAddState {
    Straight,
    Curved,
    Empty
}

pub fn path_to_fig(path: &Expr, env: &mut IEnvironment) -> Result<Figure, Box<dyn std::error::Error>> {
    match path {
        Expr::PathOperation { lhs, rhs, operator } => {
            let lines = path_to_fig_helper(lhs, operator, Some(rhs), (vec![], ToAddState::Empty), vec![], env)?;
            Ok(Figure::from(lines))
        }
        _ => unreachable!()
    }
}

fn path_to_fig_helper(
    p1: &Box<Expr>, 
    path_t: &PathOperator, 
    rest: Option<&Box<Expr>>,  
    mut to_add: (Vec<Point>, ToAddState),
    mut lines: Vec<Line>,
    env: &mut IEnvironment
) -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let p1 = p1.interpret(env)?;
    let to_add = match (p1, path_t, to_add.1) {
        (Value::Point(p1), _, ToAddState::Straight) => {
            let mut points = to_add.0;
            points.push(p1); 
            lines.push(Line::new(points));
            (vec![], ToAddState::Empty)
        }
        (Value::Point(p1), PathOperator::Curve, ToAddState::Curved) => {
            to_add.0.push(p1);
            (to_add.0, to_add.1)
        },
        (Value::Point(p1), t, ToAddState::Empty) => {
            (vec![p1], match t {
                PathOperator::Line => ToAddState::Straight,
                PathOperator::Curve => ToAddState::Curved,
            })
        },
        (Value::Path(path),t,_) => {
            let (lines_new, new_to_add) = add_path(path, to_add, env);
            lines_new.iter().for_each(|line| lines.push(line));
            new_to_add
        }, 
        _ => to_add
    };

    match rest {
        Expr::Variable(_) => todo!(),
        Expr::Point(expr, expr1) => todo!(),
        Expr::PathOperation { lhs, rhs, operator } => todo!(),
    }
}
    

fn add_path(
    fig: Figure, 
    to_add: (Vec<Point>, ToAddState), 
    env: &mut IEnvironment
) -> (Vec<Line>, (Vec<Point>, ToAddState)) {
    let mut lines: &[Line] = fig.get_lines();

    //Check if toAddState contains curved lines and if the first line in the path 
    //also contains curves
    if lines.get(0).is_some_and(|l| l.get_points().len() > 2) 
    && to_add.1 == ToAddState::Curved {
        //todo: add logic

        lines = &lines[1..];
    }
    let mut return_lines: Vec<Line> = vec![];
    if to_add.1 != ToAddState::Empty {
        return_lines.push(Line::new(to_add.0));
    }
    //Check if the last line is curved 
    if lines.iter().last().is_some_and(|l| l.get_points().len() > 2) {
        //todo: add logic

        lines = &lines[..lines.len()-1];
    }

    //todo: add logic
    return_lines.extend(lines.to_vec().into_iter());

    (return_lines, to_add)
}