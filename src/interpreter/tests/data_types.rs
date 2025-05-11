use crate::interpreter::data_types::{line::Line, point::Point};
use super::*;


/***** Point *****/
#[test] 
fn point_addition() {
    let i1: Point = (5,5).into();
    let i2 = (20,20).into();
    assert_eq!(i1+i2, (25,25).into())
}
#[test] 
fn point_subtraction() {
    let i1 = (5,5).into();
    let i2: Point = (20,20).into();
    assert_eq!(&i2 - &i1, (15,15).into())
}


/***** Line *****/
#[test]
fn get_first_and_last_point() {
    let i1 = Line::Curved(vec![(0,0).into(), (5,5).into(), (10,10).into()]);
    assert_eq!(i1.get_first_point().unwrap().clone(), (0,0).into());
    assert_eq!(i1.get_last_point().unwrap().clone(), (10,10).into());
}

#[test]
fn insert_first_and_last_point() {
    let mut i1 = Line::Curved(vec![(0,0).into(), (5,5).into(), (10,10).into()]);
    i1.insert_point_first((20,20).into());
    assert_eq!(i1.get_first_point().unwrap().clone(), (20,20).into());
    i1.insert_point_last((50,50).into());
    assert_eq!(i1.get_last_point().unwrap().clone(), (50,50).into());
}


/***** Figure *****/
#[test]
fn test_height_width() {
    let i1 = basic_square().get_shape().unwrap().get_figures().first().unwrap().clone();
    assert_eq!(i1.get_height(), 1.into());
    assert_eq!(i1.get_width(), 1.into());
    
    let i2 = basic_triangle().get_shape().unwrap().get_figures().first().unwrap().clone();
    assert_eq!(i2.get_height(), 2.into());
    assert_eq!(i2.get_width(), 2.into());
}

#[test]
fn max_min() {
    let i1 = basic_square().get_shape().unwrap().get_figures().first().unwrap().clone();
    assert_eq!(i1.get_max_x(), 1.into());
    assert_eq!(i1.get_min_x(), 0.into());
    assert_eq!(i1.get_max_y(), 1.into());
    assert_eq!(i1.get_min_y(), 0.into());

    let i1 = basic_triangle().get_shape().unwrap().get_figures().first().unwrap().clone();
    assert_eq!(i1.get_max_x(), 2.into());
    assert_eq!(i1.get_min_x(), 0.into());
    assert_eq!(i1.get_max_y(), 2.into());
    assert_eq!(i1.get_min_y(), 0.into());
}

#[test] 
fn get_pop_last_first_line() {
    let mut i1 = basic_square().get_shape().unwrap().get_figures().first().unwrap().clone();
    assert_eq!(i1.pop_first_line().unwrap(), Line::Straight(vec![(0, 0).into(), (1,0).into()]));
    assert_eq!(*i1.get_first_line().unwrap(), Line::Straight(vec![(1, 0).into(), (1,1).into()]));
    assert_eq!(i1.pop_last_line().unwrap(), Line::Straight(vec![(0, 1).into(), (0,0).into()]));
    assert_eq!(*i1.get_last_line().unwrap(), Line::Straight(vec![(1, 1).into(), (0,1).into()]));
}


/***** Figure Array *****/
#[test]
fn top_left() {
    let i1 = basic_house().get_shape().unwrap();
    assert_eq!(i1.get_top_left(), (0,2).into());

    let i2 = basic_square().get_shape().unwrap();
    assert_eq!(i2.get_top_left(), (0,1).into());
    
    let i3 = basic_triangle().get_shape().unwrap();
    assert_eq!(i3.get_top_left(), (0,2).into());
}

#[test]
fn min_max_shape() {
    let i1 = basic_house().get_shape().unwrap().clone();
    assert_eq!(i1.max_x(), 1.into());
    assert_eq!(i1.min_x(), 0.into());
    assert_eq!(i1.max_y(), 2.into());
    assert_eq!(i1.min_y(), 0.into());
}

#[test]
fn height_width() {
    let i1 = basic_house().get_shape().unwrap().clone();
    assert_eq!(i1.height(), 2.into());
    assert_eq!(i1.width(), 1.into());
}

