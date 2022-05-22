use circuit::{lexer, parser::ParseStream};
use circuit::parser::parse;

const CODE: &str = r#"
cos(2) * 5
"#;

fn main() {
    let tokens = lexer::tokenize(CODE).map_err(|errors| format!("Unable tokenize input: {:?}", errors.into_iter().map(|error| error.details))).unwrap();
    println!("Tokens {:#?}", tokens);
    //println!("{:#?}", tokens.collect::<Vec<Token>>());
    let mut parse_stream = ParseStream::new(tokens);
    
    println!("AST {:#?}", parse::add(&mut parse_stream));
}
