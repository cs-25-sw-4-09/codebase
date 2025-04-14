use codebase::program::program;
#[test]
fn test_program_new_converts_ast_to_program_int() {
    let code = "begin
    x: int = 1;";

    let program = program::Program::new(code);

}