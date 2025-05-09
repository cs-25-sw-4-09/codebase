use crate::program::program::Program;
use super::{
    ExecuteP, 
    environment::REnvironment
};



impl ExecuteP for Program {
    fn run(&mut self) {
        let mut env = REnvironment::new();
        
        let decls = &self.decl_f;
        decls.iter().for_each(|decl| {
            
        });

        let stmts = &self.stmts;
        stmts.iter().for_each(|stmt| {
            
        });

        
    }
}