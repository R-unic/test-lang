use crate::parser::Parser;


pub fn interpret(source: &str) {
  let mut parser = Parser::new(source);
  let ast = parser.parse();
  // println!("{}", ast);
}
