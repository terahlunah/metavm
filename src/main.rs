use mana::lang::{
    lexer::Lexer,
    token::{Token, TokenKind},
};

fn main() {
    let source = r#"
def fact2 =
    countTo       # produce a range from 0 to n (not included)
    \* reduce   # consume an seq and calculate the product of its elements
"#;

    let mut lexer = Lexer::new(source);

    loop {
        match lexer.next() {
            Ok(Token {
                kind: TokenKind::Eof,
                ..
            }) => {
                println!("EOF");
                break;
            }
            Ok(t) => {} //println!("{:?}", t)
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}
