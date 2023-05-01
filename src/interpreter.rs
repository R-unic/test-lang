use crate::{lexer::{tokenize}, lexer::token::Token};

pub fn interpret(source: &str) {
  let tokens: Vec<Token> = tokenize(&source);
  for token in tokens.iter() {
    println!("{}", token.to_string());
  }
}
