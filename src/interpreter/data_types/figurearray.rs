use super::{figure::Figure};

pub struct FigureArray(Vec<Figure>);


impl FigureArray {
    pub fn get_mut_lines(&mut self) -> &mut Vec<Figure> {
        &mut self.0 
    }

}