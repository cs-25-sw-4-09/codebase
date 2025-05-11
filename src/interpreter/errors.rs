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


#[derive(Debug, Clone)]
pub struct PolyPathNotFound;
impl Error for PolyPathNotFound {}
impl fmt::Display for PolyPathNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Could not find polygon or path for shapeconstructor", 
        )
    }
}

#[derive(Debug, Clone)]
pub struct ArrayEmpty(pub String);
impl Error for ArrayEmpty {}
impl fmt::Display for ArrayEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Array {} is empty", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ArrayNonExcisting(pub String);
impl Error for ArrayNonExcisting {}
impl fmt::Display for ArrayNonExcisting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Array {} is not declared", self.0)
    }
}


#[derive(Debug, Clone)]
pub struct NoLinesInFigure;
impl Error for NoLinesInFigure {}
impl fmt::Display for NoLinesInFigure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Figure is empty")
    }
}

#[derive(Debug, Clone)]
pub struct NoFiguresInFigureArray;
impl Error for NoFiguresInFigureArray {}
impl fmt::Display for NoFiguresInFigureArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"No figures found")
    }
}


#[derive(Debug, Clone)]
pub struct MinCanNotBeFound;
impl Error for MinCanNotBeFound {}
impl fmt::Display for MinCanNotBeFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"min can not be found")
    }
}

#[derive(Debug, Clone)]
pub struct MaxCanNotBeFound;
impl Error for MaxCanNotBeFound {}
impl fmt::Display for MaxCanNotBeFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"max can not be found")
    }
}

#[derive(Debug, Clone)]
pub struct ArrayOutOfBounds(pub String);
impl Error for ArrayOutOfBounds {}
impl fmt::Display for ArrayOutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Array {} is out of bounds", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct InvalidArrayAccess(pub String);
impl Error for InvalidArrayAccess {}
impl fmt::Display for InvalidArrayAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{} is not an array", self.0)
    }
}


