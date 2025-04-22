use std::{collections::HashMap, error::Error};

use crate::program::r#type::Type;

use super::errors;

#[derive(Debug, Clone)]
pub struct TEnvironment {
    v_table: HashMap<String, EType>,
    f_table: HashMap<String, (Vec<Type>, Type)>,
    s_table: HashMap<String, HashMap<String, Type>>,
    r_type: Type,
}

impl TEnvironment {
    pub fn new() -> Self {
        TEnvironment {
            v_table: HashMap::new(),
            f_table: HashMap::new(),
            s_table: HashMap::new(),
            r_type: Type::Int,
        }
    }

    pub fn vtable_lookup(&self, identifier: &String) -> Result<&Type, Box<dyn Error>> {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Normal(t) | EType::Decl(t) => Ok(t),
            }
        } else {
            Err(errors::IdentifierNotFound(identifier.to_owned()).into())
        }
    }

    pub fn vtable_set(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::Normal(r#type));
    }

    pub fn vdtable_lookup(&self, identifier: &String) -> Result<&Type, Box<dyn Error>> {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Decl(t) => Ok(t),
                EType::Normal(_) => Err(errors::IdentifierNotFound(identifier.to_owned()).into()),
            }
        } else {
            Err(errors::IdentifierNotFound(identifier.to_owned()).into())
        }
    }

    pub fn vdtable_set(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::Decl(r#type));
    }

    // pub fn vtable_contains(&self, identifier: &String) -> bool {
    //     self.v_table.contains_key(identifier)
    // }

    // pub fn vdtable_contains(&self, identifier: &String) -> bool {
    //     if let Some(etype) = self.v_table.get(identifier) {
    //         match etype {
    //             EType::Decl(_) => true,
    //             EType::Normal(_) => false,
    //         }
    //     } else {
    //         false
    //     }
    // }

    pub fn vdtable_get_hashmap(&self) -> HashMap<String, Type> {
        self.v_table
            .iter()
            .filter_map(|(param_name, param_type)| match param_type {
                EType::Normal(_) => None,
                EType::Decl(r#type) => Some((param_name.clone(), r#type.clone())),
            })
            .collect()
    }

    // pub fn ftable_contains(&self, identifier: &String) -> bool {
    //     self.f_table.contains_key(identifier)
    // }

    pub fn ftable_set(&mut self, identifier: String, parameters: Vec<Type>, return_type: Type) {
        self.f_table.insert(identifier, (parameters, return_type));
    }

    pub fn ftable_lookup(&self, identifier: &String) -> Result<&(Vec<Type>, Type), Box<dyn Error>> {
        if let Some(lookup) = self.f_table.get(identifier) {
            Ok(lookup)
        } else {
            Err(errors::IdentifierNotFound(identifier.to_owned()).into())
        }
    }

    // pub fn stable_contains(&self, identifier: &String) -> bool {
    //     self.s_table.contains_key(identifier)
    // }

    pub fn stable_lookup(&self, identifier: &String) -> Result<&HashMap<String, Type>, Box<dyn Error>> {
        if let Some(lookup) = self.s_table.get(identifier) {
            Ok(lookup)
        } else {
            Err(errors::IdentifierNotFound(identifier.to_owned()).into())
        }
    }

    pub fn stable_set(&mut self, identifier: String, parameters: HashMap<String, Type>) {
        self.s_table.insert(identifier, parameters);
    }

    pub fn clone(&self) -> Self {
        let mut new = Clone::clone(self);
        new.v_table.clear(); // TODO: Should this reset more? E.g. return type
        new
    }

    pub fn return_set(&mut self, r#type: Type) {
        self.r_type = r#type;
    }

    pub fn return_lookup(&self) -> Type {
        self.r_type
    }
}

#[derive(Debug, Clone)]
pub enum EType {
    Normal(Type),
    Decl(Type),
}
