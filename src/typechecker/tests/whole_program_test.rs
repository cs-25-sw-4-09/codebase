use crate::{program::program::Program, typechecker::TypeCheckP};

use super::valid_programs::get_programs;

/* Example Program parsing */
#[test]
fn example_program_parsing() {
    //A program is succesfully parsed if get_root() does not panic.
    get_programs()
    .into_iter()
    .for_each(|el| {
        let string = el.to_string();
        let mut program = Program::new(&string).unwrap();
        let idkthing = program.type_check();
        //println!("AGFHUIOPUGFDHJKFDSGJKLUDRDGHIJOUTRESFGUJIOUTDSFHJIOUTRDYHIJOUTDTYUI {:?}", idkthing.err());
        //assert!(1==2)
        assert!(idkthing.is_ok());
    });
}