use circuit::{lexer::{self, token::{Token, TokenKind}}, parser::{ParseStream, ast::*}};
use circuit::parser::circuit as parse;

const CODE: &str = r#"
"#;
// FIXME: ASD
fn main() {
    let tokens = lexer::tokenize(CODE).unwrap();
    println!("Tokens {:#?}", tokens);
    //println!("{:#?}", tokens.collect::<Vec<Token>>());
    let mut parse_stream = ParseStream::new(tokens);
    println!("AST {:#?}", parse::property(&mut parse_stream));
}
