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

#[derive(Debug, Clone)]
pub struct DeclValueNotSpecified(pub String);
impl Error for DeclValueNotSpecified {}
impl fmt::Display for DeclValueNotSpecified {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Decl {}'s value is not specified", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct DivideByZero;
impl Error for DivideByZero {}
impl fmt::Display for DivideByZero {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Can't divide by 0")
    }
}

