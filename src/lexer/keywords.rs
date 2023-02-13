use super::{syntax::Syntax, token::PossibleTokenValue};

pub fn is_keyword(s: &str) -> bool {
  let keywords = vec!["true", "false", "none", "function", "if", "else", "for", "foreach", "while", "break", "next", "match", "global"];
  keywords.contains(&s)
}

pub fn get_keyword_syntax(s: &str) -> Result<Syntax, ()> {
  match s {
    "function" => Ok(Syntax::Function),
    "if" => Ok(Syntax::If),
    "else" => Ok(Syntax::Else),
    "for" => Ok(Syntax::For),
    "foreach" => Ok(Syntax::ForEach),
    "while" => Ok(Syntax::While),
    "break" => Ok(Syntax::Break),
    "next" => Ok(Syntax::Next),
    "match" => Ok(Syntax::Match),
    "global" => Ok(Syntax::Global),
    "true" | "false" => Ok(Syntax::Boolean),
    "none" => Ok(Syntax::None),
    _ => Err(eprintln!("{} is not a keyword", s.to_string()))
  }
}

pub fn get_keyword_value(s: &str) -> Option<PossibleTokenValue> {
  match s {
    "true" => Some(PossibleTokenValue::Boolean(true)),
    "false" => Some(PossibleTokenValue::Boolean(false)),
    "none" => Some(PossibleTokenValue::None(())),
    _ => None
  }
}
