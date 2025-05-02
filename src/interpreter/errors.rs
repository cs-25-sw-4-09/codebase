use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct FunctionNotReturning(pub String);
impl Error for FunctionNotReturning {}
impl fmt::Display for FunctionNotReturning {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Function {} did not return", self.0)
    }
}