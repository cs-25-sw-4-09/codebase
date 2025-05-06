use crate::program::{expression::Expr, figure::{Figure, Point}, operators::pathoperator::PathOperator};


pub fn path_to_fig(p1: &Box<Expr>, op: &PathOperator, rest: &Box<Expr>) -> Figure {
    let mut fig = Figure::new();
    path_to_fig_helper(p1, op, rest, &mut fig, vec![]);
    fig
}

fn path_to_fig_helper(
    p1: &Box<Expr>, 
    op: &PathOperator, 
    rest: &Box<Expr>, 
    fig: &mut Figure, 
    mut to_add: Vec<Point>
) {
    match (rest.as_ref(), op, to_add.is_empty()) {
        (Expr::PathOperation { lhs, rhs, operator }, PathOperator::Line, true) => {
            let (p2, rest) = (lhs, rhs);
            fig.push_line(vec![Point::from(p1), Point::from(p2)]);
            path_to_fig_helper(p2, operator, rest, fig, vec![]);
        }
        (Expr::PathOperation { lhs, rhs, operator }, PathOperator::Line, false) => {
            let (p2, rest) = (lhs, rhs);
            let p1 = Point::from(p1);
            to_add.push(p1.clone());
            fig.push_line(to_add.to_vec());
            fig.push_line(vec![p1, Point::from(p2)]);
            path_to_fig_helper(p2, operator, rest, fig, vec![]);
        }
        (Expr::PathOperation { lhs, rhs, operator }, PathOperator::Curve, _) => {
            let (p2, rest) = (lhs, rhs);
            to_add.push(Point::from(p1));
            path_to_fig_helper(p2, operator, rest, fig, to_add);
        },
        //todo: tilføj error handling, potentielt
        _ => ()
    }   
}