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

#[derive(Debug, Clone)]
pub struct TooManyPoints(pub String);
impl Error for TooManyPoints {}
impl fmt::Display for TooManyPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SVG only supports bezier curves with a maximum of 4 points. Amount used: {}", self.0)
    }
}