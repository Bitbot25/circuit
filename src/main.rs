extern crate circuit_lang as circuit;

use circuit::{lexer, parser::ParseStream};
use circuit::parser::parse;

const CODE: &str = r#"
fun hello() {
    print("Cool");
    1+2;
}
"#;

fn main() {
    let tokens = lexer::tokenize(CODE).map_err(|errors| format!("Unable tokenize input: {:?}", errors.into_iter().map(|error| error.details))).unwrap();
    println!("Tokens {:#?}", tokens);
    //println!("{:#?}", tokens.collect::<Vec<Token>>());
    let mut parse_stream = ParseStream::new(tokens, CODE);
    
    println!("AST {:#?}", parse::statement(&mut parse_stream));
}
