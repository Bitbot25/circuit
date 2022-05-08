use std::collections::HashMap;

use circuit::interpreter::environment::Environment;
use circuit::interpreter::object::CircuitObject;
use circuit::lexer;
use circuit::parser::circuit::Circuit;

const CODE: &str = r#"
asd.to_string();
"#;

fn main() {
    let tokens = lexer::tokenize(CODE);
    let ast = Circuit::new(tokens).parse();
    println!("{:#?}", ast);
}