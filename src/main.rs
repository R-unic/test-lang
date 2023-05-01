mod logger;
mod lexer;
mod parser;
mod interpreter;
mod cli;

fn main() -> () {
  cli::run();
}
