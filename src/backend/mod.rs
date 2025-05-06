pub mod environment;
pub mod value_type;
pub mod program;

pub trait ExecuteP {
    fn run(&mut self);
}