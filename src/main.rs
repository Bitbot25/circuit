use circuit::lexer;
use circuit::lexer::token::TokenKind;
use circuit::parser::circuit::Circuit;
use circuit::span::{FileIndex, SpanStack};

const CODE: &str = r#"
a
"#;
// FIXME: ASD
fn main() {
    let tokens = lexer::tokenize(CODE);
    let res = Circuit::new(tokens).expect(TokenKind::Bang, "Expected '!'");
    if let Err(error) = res {
        println!("{}", error.span().display(CODE));
    }
}
