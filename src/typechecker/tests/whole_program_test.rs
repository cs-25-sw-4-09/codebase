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
        //println!("{:?}",program.type_check().err());
        //assert!(program.type_check().is_ok())
        
    });
}