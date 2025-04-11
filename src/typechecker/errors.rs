use core::fmt;
use std::error::Error;

use crate::program::r#type::Type;

type Result<'a, T> = std::result::Result<T, IdentifierNotFound<'a>>;

#[derive(Debug, Clone)]
pub struct IdentifierNotFound<'a>(pub &'a String);
impl<'a> Error for IdentifierNotFound<'a> {}
#[derive(Debug, Clone)]
pub struct BinaryOperationTypeNotCompatible(pub Type, pub Type);
impl Error for BinaryOperationTypeNotCompatible {}
#[derive(Debug, Clone)]
pub struct UnaryOperationTypeNotCompatible(pub Type);
impl Error for UnaryOperationTypeNotCompatible {}
#[derive(Debug, Clone)]
pub struct FCallParametersIncompatible<'a>(pub &'a String);
impl<'a> Error for FCallParametersIncompatible<'a> {}
#[derive(Debug, Clone)]
pub struct SCallParametersIncompatible<'a>(pub &'a String);
impl<'a> Error for SCallParametersIncompatible<'a> {}

impl<'a> fmt::Display for IdentifierNotFound<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not find identifier: {}", self.0)
    }
}

impl fmt::Display for BinaryOperationTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binary operation types not compatible: {:?} & {:?}", self.0, self.1)
    }
}

impl fmt::Display for UnaryOperationTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unary operation type not compatible: {:?}", self.0)
    }
}

impl<'a> fmt::Display for FCallParametersIncompatible<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function call identifier {}'s parameter types is incompatible", self.0)
    }
}

impl<'a> fmt::Display for SCallParametersIncompatible<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shape constructor call identifier {}'s parameter types is incompatible", self.0)
    }
}