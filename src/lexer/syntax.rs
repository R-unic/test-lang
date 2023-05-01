#[derive(Copy, Clone, std::fmt::Debug)]
pub enum Syntax {
  Identifier, Float, String, Char, Boolean, None,

  Plus, PlusEqual, Minus, MinusEqual, Star, StarEqual, Slash, SlashEqual, Carat, CaratEqual, Percent, PercentEqual,
  Less, LessEqual, Greater, GreaterEqual, Equal, EqualEqual, Bang, BangEqual,
  Ampersand, Pipe, PipeEqual,
  ColonColon, Colon, Dot, LeftBrace, RightBrace, LeftBracket, RightBracket, LeftParen, RightParen, Comma,
  Hashtag, HyphenArrow,

  Function, If, Else, For, ForEach, While, Global, Constant, Break, Next, Match,

  EOF,
}