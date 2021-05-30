mod lexer;
mod parser;

use std::path::PathBuf;

use lexer::Lexer;
// use parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lang_file = PathBuf::from("./snippets/echo.qn");
    let mut lexer = Lexer::from(lang_file)?;
    let tokens = lexer.lex();

    println!("Tokens: {:#?}", tokens);

    // let _parser = Parser::new(tokens);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    #[test]
    fn echo_test() -> Result<(), Box<dyn std::error::Error>> {
        let lang_file =
            std::fs::read_to_string("/Users/codekid/Projects/Rust/quen/snippets/echo.qn")?;
        let mut l = Lexer::from_contents(lang_file);
        let tokens = l.lex();
        let first = tokens.first().unwrap();
        assert_eq!(*first.token(), Token::Comment);
        Ok(())
    }
}
