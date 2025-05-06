use std::collections::HashMap;
use super::value_type::VType;


pub struct REnvironment {
    v_table: HashMap<String, VType>,
    //f_table: HashMap<String, TypP>,
    //s_table: HashMap<String, Type>,
    //r_type: Option<VType>,
}

impl REnvironment {
    pub fn new() -> Self {
        Self {
            v_table: HashMap::new()
        }
    }


    pub fn vtable_set(&mut self, identifier: String, value: VType) {
        self.v_table.insert(identifier.to_string(), value);   
    }
    

    pub fn vtable_lookup(&self, identifier: &str) -> Option<&VType> {
        self.v_table.get(identifier)
    }

    pub fn vtable_lookup_unwrap(&self, identifier: &str) -> &VType {
        self.v_table.get(identifier).unwrap()
    }

    
}

