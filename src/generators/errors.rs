use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AttributeNotValid(pub String);
impl Error for AttributeNotValid {}
impl fmt::Display for AttributeNotValid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Attribute {} not valid", self.0)
    }
}