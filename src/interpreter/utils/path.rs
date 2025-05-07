
use crate::{
    interpreter::{
        environment::IEnvironment, InterpretE, 
        data_types::{figure::{Figure, Line}, point::Point}
    }, 
    program::{expression::Expr, operators::pathoperator::PathOperator}};
use crate::interpreter::value::Value;

#[derive(PartialEq)]
enum ToAdd {
    Straight(Point),
    Curved(Vec<Point>),
    Empty
}

pub fn path_to_fig(path: &Expr, env: &mut IEnvironment) -> Result<Figure, Box<dyn std::error::Error>> {
    match path {
        Expr::PathOperation { lhs, rhs, operator } => {

            println!("lhs: {:?}\n operator: {:?}\n rhs {:?}\n\n", lhs, operator, rhs);

            let lines = path_to_fig_helper(lhs, Some(operator), Some(rhs), ToAdd::Empty, vec![], env)?;
            println!("{:?}", lines);
            Ok(Figure::from(lines))
        }
        _ => unreachable!()
    }
}

fn path_to_fig_helper(
    p1: &Box<Expr>, 
    path_t: Option<&PathOperator>, 
    rest: Option<&Box<Expr>>,  
    to_add: ToAdd,
    mut lines: Vec<Line>,
    env: &mut IEnvironment
) -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let p1 = p1.interpret(env)?;

    //Todo: check if there is missing logic
    //Handle to_add logic --> If there is points in it, then it should be handled correctly
    let to_add = match (p1, path_t, to_add) {
        (Value::Point(p2), _, ToAdd::Straight(p1)) => {
            lines.push(Line::from((p1, p2)));
            ToAdd::Empty
        }
        (Value::Point(p1), Some(PathOperator::Curve), ToAdd::Curved(mut points)) => {
            points.push(p1);
            ToAdd::Curved(points)
        },
        (Value::Point(p1), Some(t), ToAdd::Empty) => {
            match t {
                PathOperator::Line => ToAdd::Straight(p1),
                PathOperator::Curve => ToAdd::Curved(vec![p1]),
            }
        },
        (Value::Path(path), _t, to_add) => {
            //todo: handle T 
            let (lines_new, new_to_add) = add_path(path, to_add);
            lines_new.into_iter().for_each(|line| lines.push(line));
            new_to_add
        }, 
        (_,_,to_add) => to_add
    };

    //Handle the recursion
    if let Some(rest) = rest {
        match rest.as_ref() {
            Expr::PathOperation { lhs, rhs, operator } => {
                path_to_fig_helper(lhs, Some(operator), Some(rhs), to_add, lines, env)
            }
            val => path_to_fig_helper(&Box::new(val.clone()), None, None, to_add, lines, env), 
        }
    } else {
        Ok(lines)
    }    
}
    

fn add_path(
    fig: Figure, 
    to_add: ToAdd, 
) -> (Vec<Line>, ToAdd) {
    let mut lines: &[Line] = fig.get_lines();
    let mut return_lines: Vec<Line> = vec![];

    //First handle toAdd 
    match (to_add, lines.get(0)) {
        (ToAdd::Straight(point), Some(line)) => return_lines.push(Line::from((point, line.get_points()[0].clone()))),
        (ToAdd::Curved(mut points), Some(line)) if line.get_points().len() > 2 =>  {
            //If the first element already is also curved, then they should extend each other
            points.extend(line.get_points().to_vec().into_iter());
            return_lines.push(Line::from(points));
            
            //First point has already been used
            lines = &lines[1..];
        } 
        (ToAdd::Curved(mut points), Some(line)) => {
            points.push(line.get_points()[0].clone());
            return_lines.push(Line::from(points));
        }
        (_, _) => (),
    }

    //Add all middle lines in the path
    return_lines.extend(lines.to_vec().into_iter().take(lines.len()-1));
    
    //Check if the last line is curved and if it is then return it
    let return_add = match lines.iter().last() {
        Some(line) if line.get_points().len() > 2 => ToAdd::Curved(line.get_points().to_vec()),
        Some(line) => {
            return_lines.push(line.clone());
            ToAdd::Empty
        }
        None => ToAdd::Empty 
    };

    (return_lines, return_add)
}