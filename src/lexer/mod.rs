pub mod syntax;
pub mod token;
mod keywords;

use std::num::ParseFloatError;

use crate::logger::Logger;
use self::keywords::{is_keyword, get_keyword_syntax, get_keyword_value, is_type_keyword, get_type_syntax};
use self::syntax::Syntax;
use self::token::{Token, PossibleTokenValue};

fn f64_from_str(s: &str, radix: u32) -> Result<f64, ParseFloatError> {
  let float_val: Result<f64, ParseFloatError> = s.parse::<f64>();
  match i64::from_str_radix(&s, radix) {
    Ok(val) => Ok(val as f64),
    Err(_) => Err(float_val.err().unwrap()),
  }
}

pub struct Lexer {
  position: usize,
  line: usize,
  chars: Vec<char>,
  tokens: Vec<Token>,
  pub logger: Logger
}

impl Lexer {
  pub fn new(source: &str) -> Self {
    Self {
      position: 0,
      line: 1,
      tokens: vec![],
      chars: source.chars().collect(),
      logger: Logger::new()
    }
  }

    /// make sure finished() returns false when using!
  fn peek(&self, offset: usize) -> char {
    self.chars.get(self.position + offset).copied().unwrap()
  }

  fn current_char(&self) -> char {
    self.peek(0)
  }

  fn match_char(&mut self, expected: char) -> bool {
    if self.finished() { return false; }
    if !self.char_exists(1) { return false; }
    if self.peek(1) != expected { return false; }
    self.advance();
    true
  }

  fn finished(&self) -> bool {
    self.position >= self.chars.len()
  }

  /// you probably don't want to use this, use finished()
  fn char_exists(&self, offset: usize) -> bool {
    self.chars.get(self.position + offset).copied().is_some()
  }

  fn advance(&mut self) -> Option<&char> {
    let char: Option<&char> = self.chars.get(self.position);
    self.position += 1;
    char
  }

  fn add_token(&mut self, syntax: Syntax, value: Option<PossibleTokenValue>) -> () {
    self.tokens.push(Token {
      syntax_type: syntax,
      value: value
    });
  }

  fn is_hex(&self) -> bool {
    self.current_char() == '0' &&
    self.char_exists(1) &&
    self.peek(1) == 'x' &&
    self.char_exists(2) &&
    self.peek(2).is_digit(16)
  }

  fn is_binary(&self) -> bool {
    self.current_char() == '0' &&
    self.char_exists(1) &&
    self.peek(1) == 'b' &&
    self.char_exists(2) &&
    self.peek(2).is_digit(2)
  }

  fn skip_comments(&mut self, multiline: bool) -> () {
    self.advance();
    while !self.end_of_comment(multiline, self.line) {
      self.advance();
    }
  }

  fn end_of_comment(&mut self, multiline: bool, current_line: usize) -> bool {
    if multiline {
      self.match_char(':') &&
      self.match_char('#') &&
      self.match_char('#')
    } else {
      self.line == current_line || self.finished()
    }
  }

  fn skip_whitespace(&mut self) -> () {
    while !self.finished() && self.current_char().is_whitespace() {
      self.advance();
    }
  }

  fn read_number(&mut self) -> () {
    let mut num_str = String::new();
    let radix: u32 = if self.is_hex() {
      self.advance();
      self.advance();
      16
    } else if self.is_binary() {
      self.advance();
      self.advance();
      2
    } else { 10 };

    while !self.finished() && self.current_char().is_digit(radix) {
      num_str += &self.advance().unwrap().to_string();
    }

    let value: f64 = f64_from_str(&num_str, radix).unwrap();
    self.add_token(Syntax::Float, Some(PossibleTokenValue::Float(value)));
  }

  fn read_string(&mut self, delim: char) -> () {
    self.advance();
    let mut res_str = String::new();
    while !self.finished() && self.current_char() != delim {
      res_str += &self.advance().unwrap().to_string();
    }
    self. add_token(Syntax::String, Some(PossibleTokenValue::String(res_str)));
  }

  fn read_char(&mut self, delim: char) -> () {
    self.advance();
    let mut res_str = String::new();
    while !self.finished() && self.current_char() != delim {
      res_str += &self.advance().unwrap().to_string();
      if res_str.len() > 1 {
        self.logger.report_error("Character overflow", "Character literal has more than one character", self.position, self.line);
        break
      }
    }
    self.add_token(Syntax::Char, Some(PossibleTokenValue::Char(res_str.chars().next().unwrap())));
  }

  fn read_identifier(&mut self) -> () {
    let mut ident_str = String::new();
    while !self.finished() {
      if self.char_exists(1) && !self.peek(1).is_ascii_alphanumeric() && self.peek(1) != '_' && self.peek(1) != '$' {
        ident_str += &self.current_char().to_string();
        self.skip_whitespace();
        break;
      }
      ident_str += &self.advance().unwrap().to_string();
    }
    if is_keyword(&ident_str) {
      let syntax_type = get_keyword_syntax(&ident_str);
      let value = get_keyword_value(&ident_str);
      self.add_token(syntax_type, value);
    } else if is_type_keyword(&ident_str) {
      let syntax_type = get_type_syntax(&ident_str);
      self.add_token(syntax_type, Some(PossibleTokenValue::Type(ident_str)));
    } else {
      self.add_token(Syntax::Identifier, Some(PossibleTokenValue::String(ident_str)));
    }
  }

  fn lex(&mut self) -> () {
    let char = self.current_char();
    match char {
      '.' => self.add_token(Syntax::Dot, None),
      '{' => self.add_token(Syntax::LeftBrace, None),
      '}' => self.add_token(Syntax::RightBrace, None),
      '[' => self.add_token(Syntax::LeftBracket, None),
      ']' => self.add_token(Syntax::RightBracket, None),
      '(' => self.add_token(Syntax::LeftParen, None),
      ')' => self.add_token(Syntax::RightParen, None),
      ',' => self.add_token(Syntax::Comma, None),
      ';' => { self.advance(); },

      '\n' => self.line += 1,
      '"' => self.read_string(char),
      '\'' => self.read_char(char),

      '#' => if self.match_char('#') {
        let is_multiline = self.match_char(':');
        self.skip_comments(is_multiline);
      } else {
        self.add_token(Syntax::Hashtag, None);
      },
      ':' => {
        if self.match_char(':') {
          self.add_token(Syntax::ColonColon, None);
        } else {
          self.add_token(Syntax::Colon, None);
        }
        self.advance();
      },
      '+' => {
        if self.match_char('=') {
          self.add_token(Syntax::PlusEqual, None);
        } else {
          self.add_token(Syntax::Plus, None);
        }
        self.advance();
      },
      '-' => {
        if self.match_char('=') {
          self.add_token(Syntax::MinusEqual, None);
        } else if self.match_char('>') {
          self.add_token(Syntax::HyphenArrow, None);
        } else {
          self.add_token(Syntax::Minus, None);
        }
        self.advance();
      },
      '*' => {
        if self.match_char('=') {
          self.add_token(Syntax::StarEqual, None);
        } else {
          self.add_token(Syntax::Star, None);
        }
        self.advance();
      },
      '/' => {
        if self.match_char('=') {
          self.add_token(Syntax::SlashEqual, None);
        } else {
          self.add_token(Syntax::Slash, None);
        }
        self.advance();
      },
      '^' => {
        if self.match_char('=') {
          self.add_token(Syntax::CaratEqual, None);
        } else {
          self.add_token(Syntax::Carat, None);
        }
        self.advance();
      },
      '%' => {
        if self.match_char('=') {
          self.add_token(Syntax::PercentEqual, None);
        } else {
          self.add_token(Syntax::Percent, None);
        }
        self.advance();
      },
      '&' => {
        self.add_token(Syntax::Ampersand, None);
        self.advance();
      },
      '|' => {
        if self.match_char('=') {
          self.add_token(Syntax::PipeEqual, None);
        } else {
          self.add_token(Syntax::Pipe, None);
        }
        self.advance();
      },
      '!' => {
        if self.match_char('=') {
          self.add_token(Syntax::BangEqual, None);
        } else {
          self.add_token(Syntax::Bang, None);
        }
        self.advance();
      },
      '=' => {
        if self.match_char('=') {
          self.add_token(Syntax::EqualEqual, None);
        } else {
          self.add_token(Syntax::Equal, None);
        }
        self.advance();
      },
      '<' => {
        if self.match_char('=') {
          self.add_token(Syntax::LessEqual, None);
        } else {
          self.add_token(Syntax::Less, None);
        }
        self.advance();
      },
      '>' => {
        if self.match_char('=') {
          self.add_token(Syntax::GreaterEqual, None);
        } else {
          self.add_token(Syntax::Greater, None);
        }
        self.advance();
      },
      default_char => {
        if default_char.is_whitespace() {
          self.skip_whitespace();
          return;
        }

        let is_ident: bool = default_char.is_ascii_alphabetic() || default_char == '_' || default_char == '$';
        let is_number: bool = default_char.is_digit(10) ||
          (default_char == '0' && self.peek(1) == 'x' && self.peek(2).is_digit(16)) ||
          (default_char == '0' && self.peek(1) == 'b' && self.peek(2).is_digit(2));

        if is_number {
          self.read_number();
        } else if is_ident {
          self.read_identifier();
        } else {
          self.logger.report_error("Unexpected character", Box::leak(default_char.to_string().into_boxed_str()), self.position, self.line);
        }
      }
    }
    self.position += 1;
  }

  pub fn tokenize(&mut self) -> &Vec<Token> {
    while !self.finished() {
      self.lex();
    }

    self.add_token(Syntax::EOF, None);
    &self.tokens
  }
}
