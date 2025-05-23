use crate::{interpreter::InterpretP, program::program::Program};

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
        let idkthing = program.interpret();
        assert!(idkthing.is_ok());
    });
}