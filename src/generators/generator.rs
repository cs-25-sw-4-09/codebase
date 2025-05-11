use std::error::Error;

use crate::{interpreter::data_types::figure::Figure, program::program::Program};

use super::svggenerator::SvgGenerator;


pub trait Generator {
    fn generate(&self, draw_array: &Vec<Figure>, file_name: String) -> Result<(), Box<dyn Error>>;
}

pub fn get_generator(format: &str) -> Option<Box<dyn Generator>> {
    match format {
        "svg" => Some(Box::new(SvgGenerator)),
        _ => None,
    }
}
