use std::collections::HashMap;

use crate::program::r#type::Type;

#[derive(Debug)]
pub struct TEnvironment {
    v_table: HashMap<String, EType>,
    f_table: HashMap<String, Type>,
    s_table: HashMap<String, Type>,
    r_type: Option<Type>,
}

impl TEnvironment {
    pub fn new() -> Self {
        TEnvironment {
            v_table: HashMap::new(),
            f_table: HashMap::new(),
            s_table: HashMap::new(),
            r_type: None,
        }
    }

    pub fn vtable_lookup(&self, identifier: &String) -> Option<&Type> {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Normal(t) | EType::Decl(t) => Some(t),
            }
        } else {
            None
        }
    }

    pub fn vtable_set(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::Normal(r#type));
    }

    pub fn vdtable_lookup(&self, identifier: &String) -> Option<&Type> {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Decl(t) => Some(t),
                EType::Normal(_) => None,
            }
        } else {
            None
        }
    }

    pub fn vdtable_set(&mut self, identifier: String, r#type: Type) {
        self.v_table.insert(identifier, EType::Decl(r#type));
    }

    pub fn vtable_contains(&self, identifier: &String) -> bool {
        self.v_table.contains_key(identifier)
    }

    pub fn vdtable_contains(&self, identifier: &String) -> bool {
        if let Some(etype) = self.v_table.get(identifier) {
            match etype {
                EType::Decl(_) => true,
                EType::Normal(_) => false,
            }
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub enum EType {
    Normal(Type),
    Decl(Type),
}