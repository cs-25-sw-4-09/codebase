use crate::interpreter::data_types::point::Point;



/***** Point ******/
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
    assert_eq!(i2-i1, (15,15).into())
}


/***** Line ******/ //todo




 
/***** Figure ******/ //todo








/***** Figure Array ******/ //todo