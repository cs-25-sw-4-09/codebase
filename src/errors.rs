use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct CommandlineArgumentsNotCorrect;
impl Error for CommandlineArgumentsNotCorrect {}
impl fmt::Display for CommandlineArgumentsNotCorrect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Not correct.")
    }
}