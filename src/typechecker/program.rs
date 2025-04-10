use crate::{program::program::Program, typechecker::TypeCheckS};

use super::TypeCheckP;

impl TypeCheckP for Program {
    fn type_check(&mut self) {
        self.stmts.iter().for_each(|stmt| {
            println!(
                "{}",
                if stmt.type_check(&mut self.tenvironment).is_ok() {
                    "TRUE"
                } else {
                    "FALSE"
                }
            )
        })
    }
}