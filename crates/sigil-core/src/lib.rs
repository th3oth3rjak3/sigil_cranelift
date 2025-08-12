pub mod ast;
pub mod compiler;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub fn run(source: &str) {
    let parser = grammar::ProgramParser::new();
    match parser.parse(source) {
        Ok(ast) => {
            println!("Parsed successfully!");

            // Try to compile the first function
            if let Some(result) = compiler::compile_function(&ast[0]) {
                println!("Function returned: {}", result);
            }

            println!("AST: {:#?}", ast);
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
}
