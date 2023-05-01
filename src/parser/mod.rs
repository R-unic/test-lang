use crate::logger::Logger;
use crate::lexer::Lexer;
use crate::lexer::syntax::Syntax;
use crate::lexer::token::{Token, PossibleTokenValue};


pub struct Parser {
  position: usize,
  tokens: Vec<Token>,
  pub logger: Logger
}

impl Parser {
  pub fn new(source: &str) -> Self {
    let mut lexer = Lexer::new(&source);
    let tokens: &Vec<Token> = lexer.tokenize();

    Self {
      position: 0,
      tokens: tokens.to_vec(),
      logger: Logger::new()
    }
  }

  pub fn parse(&self) {

  }
}