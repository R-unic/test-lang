use super::{syntax::Syntax, token::PossibleTokenValue};
use std::collections::HashMap;

pub fn get_keywords() -> HashMap<&'static str, Syntax> {
  let keywords: HashMap<&str, Syntax> = [
    ("true", Syntax::Boolean),
    ("false", Syntax::Boolean),
    ("none", Syntax::None),
    ("fn", Syntax::Function),
    ("if", Syntax::If),
    ("else", Syntax::Else),
    ("for", Syntax::For),
    ("foreach", Syntax::ForEach),
    ("while", Syntax::While),
    ("break", Syntax::Break),
    ("next", Syntax::Next),
    ("match", Syntax::Match),
    ("global", Syntax::Global),
    ("const", Syntax::Constant),
  ].into();

  keywords
}

pub fn is_keyword(s: &str) -> bool {
  let keywords = &get_keywords();
  keywords.contains_key(&s)
}

pub fn get_keyword_syntax(s: &str) -> Syntax {
  let keywords = &get_keywords();
  *keywords.get(s).expect(&format!("Invalid keyword {}", s))
}

pub fn get_keyword_value(s: &str) -> Option<PossibleTokenValue> {
  match s {
    "true" => Some(PossibleTokenValue::Boolean(true)),
    "false" => Some(PossibleTokenValue::Boolean(false)),
    "none" => Some(PossibleTokenValue::None(())),
    _ => None
  }
}
