use std::{collections::HashMap, error::Error};

use crate::program::r#type::Type;

use super::errors;

#[derive(Debug, Clone)]
pub struct TEnvironment {
    v_table: HashMap<String, EType>,
    f_table: HashMap<String, (Vec<Type>, Type)>,
    s_table: HashMap<String, HashMap<String, EType>>,
    r_type: Type,
}

impl TEnvironment {
    pub fn new() -> Self {

        let mut env = TEnvironment {
            v_table: HashMap::new(),
            f_table: HashMap::new(),
            s_table: HashMap::new(),
            r_type: Type::Int,
        };

        env.stable_init();
        env.ftable_init();

        env
    }

    pub fn ftable_init(&mut self){
        self.ftable_set("push".to_string(), vec![Type::IntArray,Type::Int],Type::IntArray);
        self.ftable_set("remove".to_string(), vec![Type::IntArray,Type::Int],Type::IntArray);
    }

    pub fn stable_init(&mut self){
        let mut path_param = HashMap::new();
        path_param.insert("stroke".to_string(), EType::DeclDefault(Type::Color));
        path_param.insert("thickness".to_string(), EType::DeclDefault(Type::Int));
        self.stable_set("Path".to_string(), path_param);

        let mut poly_param = HashMap::new();
        poly_param.insert("fill".to_string(), EType::DeclDefault(Type::Color));
        poly_param.insert("stroke".to_string(), EType::DeclDefault(Type::Color));
        poly_param.insert("thickness".to_string(), EType::DeclDefault(Type::Int));
        self.stable_set("Polygon".to_string(), poly_param);
    }
    
    pub fn vtable_lookup(&self, identifier: &String) -> Result<&Type, Box<dyn Error>> {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Normal(t) | EType::DeclNonDefault(t) | EType::DeclDefault(t) => Ok(t),
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
                EType::DeclNonDefault(t) | EType::DeclDefault(t) => Ok(t),
                EType::Normal(_) => Err(errors::IdentifierNotFound(identifier.to_owned()).into()),
            }
        } else {
            Err(errors::IdentifierNotFound(identifier.to_owned()).into())
        }
    }

    pub fn vdtable_set_non_default(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::DeclNonDefault(r#type));
    }

    pub fn vdtable_set_default(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::DeclDefault(r#type));
    }

    pub fn vdtable_get_hashmap(&self) -> HashMap<String, EType> {
        self.v_table
            .iter()
            .filter_map(|(param_name, param_type)| match param_type {
                EType::Normal(_) => None,
                EType::DeclNonDefault(r#type) => Some((param_name.clone(), EType::DeclNonDefault(*r#type))),
                EType::DeclDefault(r#type)  => Some((param_name.clone(), EType::DeclDefault(*r#type))),
            })
            .collect()
    }

    pub fn stable_get_hashmap_non_default(&self, identifier: &String) -> Result<HashMap<String, EType>, Box<dyn Error>> {
        if let Some(lookup) = self.s_table.get(identifier) {
            Ok(lookup
            .iter()
            .filter_map(|(param_name, param_type)| match param_type {
                EType::Normal(_) | EType::DeclDefault(_) => None,
                EType::DeclNonDefault(r#type) => Some((param_name.clone(), EType::DeclNonDefault(*r#type))),
            })
            .collect())
        } else {
            Err(errors::IdentifierNotFound(identifier.to_owned()).into())
        }
        
    }

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

    pub fn stable_lookup(&self, identifier: &String) -> Result<&HashMap<String, EType>, Box<dyn Error>> {
        if let Some(lookup) = self.s_table.get(identifier) {
            Ok(lookup)
        } else {
            Err(errors::IdentifierNotFound(identifier.to_owned()).into())
        }
    }

    pub fn stable_set(&mut self, identifier: String, parameters: HashMap<String, EType>) {
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

    pub fn clear(&mut self) {
        self.v_table.clear();
        self.f_table.clear();
        self.s_table.clear();
    }
}

#[derive(Debug, Clone)]
pub enum EType {
    Normal(Type),
    DeclNonDefault(Type),
    DeclDefault(Type),
}

impl Default for TEnvironment {
    fn default() -> Self {
        TEnvironment::new()
    }
}
