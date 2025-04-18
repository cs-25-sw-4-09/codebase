use core::fmt;
use std::error::Error;

use crate::program::r#type::Type;

//type Result<T> = std::result::Result<T, IdentifierNotFound>; //TODO: Finde ud af om den her linje vil have nogen effekt.

#[derive(Debug, Clone)]
pub struct IdentifierNotFound(pub String);
impl Error for IdentifierNotFound {}
impl fmt::Display for IdentifierNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not find identifier: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOperationTypeNotCompatible(pub Type, pub Type);
impl Error for BinaryOperationTypeNotCompatible {}
impl fmt::Display for BinaryOperationTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Binary operation types not compatible: {:?} & {:?}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOperationTypeNotCompatible(pub Type);
impl Error for UnaryOperationTypeNotCompatible {}
impl fmt::Display for UnaryOperationTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unary operation type not compatible: {:?}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct FCallParametersIncompatible(pub String);
impl Error for FCallParametersIncompatible {}
impl fmt::Display for FCallParametersIncompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Function call identifier {}'s parameter types is incompatible", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SCallParametersIncompatible(pub String, pub String, pub Type, pub Type);
impl Error for SCallParametersIncompatible {}
impl fmt::Display for SCallParametersIncompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} shape constructor '{}', expected type {:?} - got type: {:?}", self.0, self.1, self.2, self.3)
    }
}

#[derive(Debug, Clone)]
pub struct SCallParameterNotFound(pub String, pub String);
impl Error for SCallParameterNotFound {}
impl fmt::Display for SCallParameterNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Shape constructor parameter {} not declared in {}", self.0, self.1)
    }
}



#[derive(Debug, Clone)]
pub struct IdentifierAlreadyDeclared(pub String);
impl Error for IdentifierAlreadyDeclared {}
impl fmt::Display for IdentifierAlreadyDeclared {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Identifier {} already declared", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct VariableExpressionTypeNotMatch(pub String, pub Type, pub Type);
impl Error for VariableExpressionTypeNotMatch {}
impl fmt::Display for VariableExpressionTypeNotMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Variable {}’s expression type: {:?} doesn’t match type: {:?}", self.0, self.1, self.2)
    }
}



#[derive(Debug, Clone)]
pub struct ImportAlreadyDeclared(pub String);
impl Error for ImportAlreadyDeclared {}
impl fmt::Display for ImportAlreadyDeclared {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} is already imported", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ReturnTypeNotMatch(pub Type, pub Type);
impl Error for ReturnTypeNotMatch {}
impl fmt::Display for ReturnTypeNotMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The return type {:?} doesn’t match type: {:?}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct ColorTypeNotCompatible(pub Type, pub Type,pub Type, pub Type);
impl Error for ColorTypeNotCompatible {}
impl fmt::Display for ColorTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The values of color {:?},{:?},{:?},{:?} doesn’t match type: Int", self.0, self.1,self.2,self.3)
    }
}