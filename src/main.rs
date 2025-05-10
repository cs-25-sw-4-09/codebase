use std::{env, error::Error, path::Path};

use codebase::{interpreter::{value::Value, InterpretP}, program::{expression::Expr, program::Program}, typechecker::TypeCheckP};

fn main() -> Result<(), Box<dyn Error>> {

    let mut args = env::args();
    let executable_name = args.next().unwrap();
    let file_to_parse = args.next().ok_or_else(|| format!("{} <input> <generator> [-argument value] [-argument2 value2]...", executable_name))?;
    let output_generators: Vec<String> = args.next().ok_or_else(|| format!("{} <input> <generator> [-argument value] [-argument2 value2]...", executable_name))?.split(",").map(str::to_string).collect();
    
    let mut program = Program::from_file(Path::new(file_to_parse.as_str()))?;
    
    while let (Some(mut arg_name), Some(arg_value)) = (args.next(), args.next()) {
        if arg_name.starts_with("-") { // only parse if on correct form
            arg_name = arg_name.replacen("-", "", 1); // Remove -
            let arg_value = if let Ok(b) = arg_value.parse::<bool>() { // Parse argument value to Value enum type or throw error if unsupported.
                Value::Boolean(b)
            } else if let Ok(i) = arg_value.parse::<i64>() {
                Value::Integer(i)
            } else if let Ok(f) = arg_value.parse::<f64>() {
                Value::Float(f)
            } else {
                return Err(format!("Unsupported commandline value for argument: {}", arg_name).into());
            };
            println!("Arg: {} Value: {:?}", arg_name, arg_value);
        }
    }

    if let Err(err) = program.type_check() {
        println!("[Typechecker] error: {}", err);
        return Err(err)
    }
    program.tenvironment.clear();
    println!("[Typechecker] OK");

    match program.interpret() {
        Ok(_) => println!("[Interpreter] OK"),
        Err(err) => println!("[Interpreter] error: {}", err),
    }

    println!("{:#?}", program.ienvironment);
    Ok(())
}
