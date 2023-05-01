use super::syntax::Syntax;

#[derive(Debug, Clone)]
pub enum PossibleTokenValue {
    // Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None(())
}

pub struct Token {
  pub syntax_type: Syntax,
  pub value: Option<PossibleTokenValue>,
}

impl ToString for Token {
    fn to_string(&self) -> String {
      format!("Token<syntax: {:?}, value: {:?}>", self.syntax_type, self.value)
    }
}

// impl Clone for Token {
//   fn clone(&self) -> Self {
//     Self { syntax_type: self.syntax_type, value: self.value.clone() }
//   }
// }
