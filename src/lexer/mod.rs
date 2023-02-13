pub mod syntax;
pub mod token;
mod keywords;

use std::num::ParseFloatError;

use self::keywords::{is_keyword, get_keyword_syntax, get_keyword_value};
use self::syntax::Syntax;
use self::token::{Token, PossibleTokenValue};

/// make sure finished() returns false when using!
fn peek(state: &LexerState, offset: usize) -> char {
  state.chars.get(state.current + offset).copied().unwrap()
}

fn current_char(state: &LexerState) -> char {
  peek(state, 0)
}

fn match_char(state: &mut LexerState, expected: char) -> bool {
  if finished(state) { return false; }
  if !char_exists(state, 1) { return false; }
  if peek(state, 1) != expected { return false; }
  advance(state);
  true
}

fn finished(state: &mut LexerState) -> bool {
  state.current >= state.chars.len()
}

/// you probably don't want to use this, use finished()
fn char_exists(state: &LexerState, offset: usize) -> bool {
  state.chars.get(state.current + offset).copied().is_some()
}

fn advance(state: &mut LexerState) -> Option<&char> {
  let char: Option<&char> = state.chars.get(state.current);
  state.current += 1;
  char
}

fn add_token(state: &mut LexerState, syntax: Syntax, value: Option<PossibleTokenValue>) -> () {
  state.tokens.push(Token {
    syntax_type: syntax,
    value: value
  });
}

fn f64_from_str(s: &str, radix: u32) -> Result<f64, ParseFloatError> {
  let float_val: Result<f64, ParseFloatError> = s.parse::<f64>();
  match i64::from_str_radix(&s, radix) {
    Ok(val) => Ok(val as f64),
    Err(_) => Err(float_val.err().unwrap()),
  }
}

fn is_hex(state: &mut LexerState) -> bool {
  current_char(state) == '0' && char_exists(state, 1) && peek(state, 1) == 'x' && char_exists(state, 2) && peek(state, 2).is_digit(16)
}

fn is_binary(state: &mut LexerState) -> bool {
  current_char(state) == '0' && char_exists(state, 1) && peek(state, 1) == 'b' && char_exists(state, 2) && peek(state, 2).is_digit(2)
}

fn read_number(state: &mut LexerState) -> () {
  let mut num_str = String::new();
  let radix: u32 = if is_hex(state) {
    advance(state);
    advance(state);
    16
  } else if is_binary(state) {
    advance(state);
    advance(state);
    2
  } else { 10 };

  while !finished(state) && current_char(state).is_digit(radix) {
    num_str += &advance(state).unwrap().to_string();
  }

  let value: f64 = f64_from_str(&num_str, radix).unwrap();
  add_token(state, Syntax::Float, Some(PossibleTokenValue::Float(value)));
}

fn read_string(state: &mut LexerState, delim: char) -> () {
  advance(state);
  let mut res_str = String::new();
  while !finished(state) && current_char(state) != delim {
    res_str += &advance(state).unwrap().to_string();
  }
  add_token(state, Syntax::String, Some(PossibleTokenValue::String(res_str)));
}

fn read_identifier(state: &mut LexerState) -> () {
  let mut ident_str = String::new();
  while !finished(state) {
    if char_exists(state, 1) && !peek(state, 1).is_ascii_alphanumeric() && peek(state, 1) != '_' && peek(state, 1) != '$' {
      ident_str += &current_char(state).to_string();
      if peek(state, 1).is_whitespace() {
        advance(state);
      }
      break;
    }
    ident_str += &advance(state).unwrap().to_string();
  }
  if is_keyword(&ident_str) {
    let syntax_type = get_keyword_syntax(&ident_str).unwrap();
    let value = get_keyword_value(&ident_str);
    add_token(state, syntax_type, value);
  } else {
    add_token(state, Syntax::Identifier, Some(PossibleTokenValue::String(ident_str)));
  }
}

fn skip_comments(state: &mut LexerState, multiline: bool) -> () {
  advance(state);
  let current_line = state.line;
  let end_of_comment = |s: &mut LexerState| {
    if multiline {
      match_char(s, ':') && match_char(s, '#') && match_char(s, '#')
    } else {
      s.line == current_line || finished(s)
    }
  };
  while !end_of_comment(state) {
    advance(state);
  }
}

struct LexerState {
  current: usize,
  line: usize,
  chars: Vec<char>,
  tokens: Vec<Token>
}

fn lex(state: &mut LexerState) -> () {
  let char = current_char(state);
  match char {
    '.' => add_token(state, Syntax::Dot, None),
    '{' => add_token(state, Syntax::LeftBrace, None),
    '}' => add_token(state, Syntax::RightBrace, None),
    '[' => add_token(state, Syntax::LeftBracket, None),
    ']' => add_token(state, Syntax::RightBracket, None),
    '(' => add_token(state, Syntax::LeftParen, None),
    ')' => add_token(state, Syntax::RightParen, None),
    ',' => add_token(state, Syntax::Comma, None),
    ';' => { advance(state); },

    '\n' => state.line += 1,
    '"' | '\'' => read_string(state, char),

    '#' => if match_char(state, '#') {
      let is_multiline = match_char(state, ':');
      skip_comments(state, is_multiline);
    } else {
      add_token(state, Syntax::Hashtag, None);
    },
    ':' => if match_char(state, ':') {
      add_token(state, Syntax::ColonColon, None)
    },
    '+' => {
      if match_char(state, '=') {
        add_token(state, Syntax::PlusEqual, None);
      } else {
        add_token(state, Syntax::Plus, None);
      }
      advance(state);
    },
    '-' => {
      if match_char(state, '=') {
        add_token(state, Syntax::MinusEqual, None);
      } else {
        add_token(state, Syntax::Minus, None);
      }
      advance(state);
    },
    '*' => {
      if match_char(state, '=') {
        add_token(state, Syntax::StarEqual, None);
      } else {
        add_token(state, Syntax::Star, None);
      }
      advance(state);
    },
    '/' => {
      if match_char(state, '=') {
        add_token(state, Syntax::SlashEqual, None);
      } else {
        add_token(state, Syntax::Slash, None);
      }
      advance(state);
    },
    '^' => {
      if match_char(state, '=') {
        add_token(state, Syntax::CaratEqual, None);
      } else {
        add_token(state, Syntax::Carat, None);
      }
      advance(state);
    },
    '%' => {
      if match_char(state, '=') {
        add_token(state, Syntax::PercentEqual, None);
      } else {
        add_token(state, Syntax::Percent, None);
      }
      advance(state);
    },
    '&' => {
      add_token(state, Syntax::Ampersand, None);
      advance(state);
    },
    '|' => {
      if match_char(state, '=') {
        add_token(state, Syntax::PipeEqual, None);
      } else {
        add_token(state, Syntax::Pipe, None);
      }
      advance(state);
    },
    '!' => {
      if match_char(state, '=') {
        add_token(state, Syntax::BangEqual, None);
      } else {
        add_token(state, Syntax::Bang, None);
      }
      advance(state);
    },
    '=' => {
      if match_char(state, '=') {
        add_token(state, Syntax::EqualEqual, None);
      } else {
        add_token(state, Syntax::Equal, None);
      }
      advance(state);
    },
    '<' => {
      if match_char(state, '=') {
        add_token(state, Syntax::LessEqual, None);
      } else {
        add_token(state, Syntax::Less, None);
      }
      advance(state);
    },
    '>' => {
      if match_char(state, '=') {
        add_token(state, Syntax::GreaterEqual, None);
      } else {
        add_token(state, Syntax::Greater, None);
      }
      advance(state);
    },
    default_char => {
      if default_char.is_whitespace() { return; }

      let is_ident: bool = default_char.is_ascii_alphabetic() || default_char == '_' || default_char == '$';
      let is_number: bool = default_char.is_digit(10) ||
        (default_char == '0' && peek(state, 1) == 'x' && peek(state, 2).is_digit(16)) ||
        (default_char == '0' && peek(state, 1) == 'b' && peek(state, 2).is_digit(2));

      if is_number {
        read_number(state);
      } else if is_ident {
        read_identifier(state);
      } else {
        panic!("Unexpected character: {}", default_char);
      }
    }
  }
  state.current += 1;
}

pub fn tokenize(source: &str) -> Vec<Token> {
  let mut state = LexerState {
    current: 0,
    line: 1,
    tokens: vec![],
    chars: source.chars().collect()
  };

  while !finished(&mut state) {
    lex(&mut state);
  }

  add_token(&mut state, Syntax::EOF, None);
  state.tokens
}
