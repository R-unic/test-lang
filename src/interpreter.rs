use crate::{lexer::{Lexer, token::Token}};

pub fn interpret(source: &str) {
  let mut lexer = Lexer::new(&source);
  let tokens: &Vec<Token> = lexer.tokenize();
  for token in tokens.iter() {
    println!("{}", token.to_string());
  }

  if lexer.logger.errored {
    return;
  }
}
