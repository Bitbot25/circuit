use circuit::{lexer::{self, token::{Token, TokenKind}}, parser::{ParseStream, ast::*}};
use circuit::parser::circuit as parse;

const CODE: &str = r#"
"hello
"#;
// FIXME: ASD
fn main() {
    let tokens = lexer::tokenize(CODE).unwrap();
    //println!("{:#?}", tokens.collect::<Vec<Token>>());
    let mut parse_stream = ParseStream::new(tokens);
    println!("{:?}", parse::literal(&mut parse_stream));
}
