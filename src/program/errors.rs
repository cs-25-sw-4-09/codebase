use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ASTNodeValueInvalid(pub String);
impl Error for ASTNodeValueInvalid {}
impl fmt::Display for ASTNodeValueInvalid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not get value from AST Node: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ASTNodeChildrenCountInvalid(pub usize, pub usize);
impl Error for ASTNodeChildrenCountInvalid {}
impl fmt::Display for ASTNodeChildrenCountInvalid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AST Node children expected: {}, got: {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct ASTNodeChildrenCountInvalidEither(pub usize, pub usize, pub usize);
impl Error for ASTNodeChildrenCountInvalidEither {}
impl fmt::Display for ASTNodeChildrenCountInvalidEither {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AST Node children expected: {} or {}, got: {}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone)]
pub struct ParemeterAlreadyDefined(pub String);
impl Error for ParemeterAlreadyDefined {}
impl fmt::Display for ParemeterAlreadyDefined {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parameter already defined: {}", self.0)
    }
}



#[derive(Debug, Clone)]
pub struct ExprParseAsIntegerError;
impl Error for ExprParseAsIntegerError {}
impl fmt::Display for ExprParseAsIntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Expression could not be parsed as integer")
    }
}

