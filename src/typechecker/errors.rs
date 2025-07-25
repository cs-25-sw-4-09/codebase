use core::fmt;
use std::error::Error;

use crate::program::r#type::Type;

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
        write!(
            f,
            "Binary operation types not compatible: {:?} & {:?}",
            self.0, self.1
        )
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
        write!(
            f,
            "Function call identifier {}'s parameter types is incompatible",
            self.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct FCallParametersCountError(pub String);
impl Error for FCallParametersCountError {}
impl fmt::Display for FCallParametersCountError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Function call with identifier {}'s do not have the same amount of params as declaration",
            self.0
        )
    }
}


#[derive(Debug, Clone)]
pub struct SCallParametersIncompatible(pub String, pub String, pub Type, pub Type);
impl Error for SCallParametersIncompatible {}
impl fmt::Display for SCallParametersIncompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} shape constructor '{}', expected type {:?} - got type: {:?}",
            self.0, self.1, self.2, self.3
        )
    }
}

#[derive(Debug, Clone)]
pub struct SCallParameterNotFound(pub String, pub String);
impl Error for SCallParameterNotFound {}
impl fmt::Display for SCallParameterNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Shape constructor parameter {} not declared in {}",
            self.0, self.1
        )
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
        write!(
            f,
            "Variable {}’s expression type: {:?} doesn’t match type: {:?}",
            self.0, self.1, self.2
        )
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
        write!(
            f,
            "The return type {:?} doesn’t match type: {:?}",
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct ColorTypeNotCompatible(pub Type, pub Type, pub Type, pub Type);
impl Error for ColorTypeNotCompatible {}
impl fmt::Display for ColorTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "The values of color {:?},{:?},{:?},{:?} doesn't match type: Int",
            self.0, self.1, self.2, self.3
        )
    }
}

#[derive(Debug, Clone)]
pub struct PointTypeNotCompatible(pub Type, pub Type);
impl Error for PointTypeNotCompatible {}
impl fmt::Display for PointTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "The values of point {:?},{:?} doesn't match type: Int or Float",
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct PathOperationTypeNotCompatible(pub Type, pub Type);
impl Error for PathOperationTypeNotCompatible {}
impl fmt::Display for PathOperationTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "The part of the path cannot be created, got {:?} and {:?}. Should be a Path or Point type",
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct PolyOperationTypeNotCompatible(pub Type);
impl Error for PolyOperationTypeNotCompatible {}
impl fmt::Display for PolyOperationTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "The values of polygon {:?} doesn't match type: Path \n A polygon must consitst of two points",
            self.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct ArrayElementsTypeNotCompatible(pub Type);
impl Error for ArrayElementsTypeNotCompatible {}
impl fmt::Display for ArrayElementsTypeNotCompatible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "The types in the array doesn't match, first element is {:?}",
            self.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct ArrayAssignTypeConflict(pub Type, pub Type);
impl Error for ArrayAssignTypeConflict {}
impl fmt::Display for ArrayAssignTypeConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "It is not possible to assign a type {:?} to a array type {:?}",
            self.0, self.1
        )
    }
}


#[derive(Debug, Clone)]
pub struct ArrayIndexTypeError(pub Type);
impl Error for ArrayIndexTypeError {}
impl fmt::Display for ArrayIndexTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Can not index a array with type {:?}, can only with type Int",
            self.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct NotAnArrayToIndex(pub Type);
impl Error for NotAnArrayToIndex {}
impl fmt::Display for NotAnArrayToIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Can not array index a the type \"{:?}\", can only array index array types",
            self.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct NotAMemberType(pub Type);
impl Error for NotAMemberType {}
impl fmt::Display for NotAMemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "It is not possible to make member access on the type {:?}",
            self.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct MemberAccessPoint();
impl Error for MemberAccessPoint {}
impl fmt::Display for MemberAccessPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "You can use \"x\" or \"y\" as member access on a point")
    }
}

#[derive(Debug, Clone)]
pub struct MemberAccessShape();
impl Error for MemberAccessShape {}
impl fmt::Display for MemberAccessShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "You can use \"width\" or \"height\" as member access on a shape"
        )
    }
}

#[derive(Debug, Clone)]
pub struct MemberAccessColor();
impl Error for MemberAccessColor {}
impl fmt::Display for MemberAccessColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "You can use \"r\", \"g\", \"b\", or \"a\" as member access on a color"
        )
    }
}

#[derive(Debug, Clone)]
pub struct MemberAccessArray();
impl Error for MemberAccessArray {}
impl fmt::Display for MemberAccessArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "You can use \"size\" as member access on a array type"
        )
    }
}

#[derive(Debug, Clone)]
pub struct ManipulationScaleTypeFault(pub Type, pub Type);
impl Error for ManipulationScaleTypeFault {}
impl fmt::Display for ManipulationScaleTypeFault {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Was unable to scale: \nExpected type Shape got {:?}\nExpected type Int or Float got {:?}", 
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct ManipulationRotateTypeFault(pub Type, pub Type);
impl Error for ManipulationRotateTypeFault {}
impl fmt::Display for ManipulationRotateTypeFault {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Was unable to rotate: \nExpected type Shape got {:?}\nExpected type Int or Float got {:?}", 
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct ManipulationPlaceTypeFault(pub Type, pub Type);
impl Error for ManipulationPlaceTypeFault {}
impl fmt::Display for ManipulationPlaceTypeFault {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Was unable to Place shapes: Expected type {:?} got {:?}", 
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct PolyPathNotFound();
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
pub struct DrawTypeFault(pub Type, pub Type);
impl Error for DrawTypeFault {}
impl fmt::Display for DrawTypeFault {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Was unable to Draw: Expected type {:?} got {:?}", 
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct AssignTypesNoMatch(pub Type, pub Type);
impl Error for AssignTypesNoMatch {}
impl fmt::Display for AssignTypesNoMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Was unable to assign Expected type {:?} got {:?}", 
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct ForLoopCounterDeclared();
impl Error for ForLoopCounterDeclared {}
impl fmt::Display for ForLoopCounterDeclared {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Was unable make for loop as the counter is already declared"
        )
    }
}

#[derive(Debug, Clone)]
pub struct ForLoopTypeError(pub Type, pub Type);
impl Error for ForLoopTypeError {}
impl fmt::Display for ForLoopTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Was unable make for loop as the range was not of type Int, Int but {:?}, {:?}", self.0,self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct ForkNotBooltypeError(pub Type);
impl Error for ForkNotBooltypeError {}
impl fmt::Display for ForkNotBooltypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "One condition in the fork was not a Bool, found: {:?}", self.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct ErrorInPush(pub Type, pub Type);
impl Error for ErrorInPush {}
impl fmt::Display for ErrorInPush {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Wrong parameters for push expected same type but in array: {:?} {:?}", self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct ErrorInRemove(pub Type, pub Type);
impl Error for ErrorInRemove {}
impl fmt::Display for ErrorInRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Wrong parameters for remove expected array type (without empty) and int got: {:?} {:?}", self.0, self.1
        )
    }
}

