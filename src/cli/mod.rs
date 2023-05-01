use clap::{Command, Arg, ArgMatches};
use rustyline::{Editor, error::ReadlineError};
use rainbow_text::Rainbow;

use crate::interpreter::interpret;

fn read_file(path: &str) -> Option<String> {
  let contents_res: Result<String, std::io::Error> = std::fs::read_to_string(path);
  match contents_res {
    Ok(contents) => Some(contents),
    Err(e) => {
      eprintln!("Failed to read file '{}': {}", path, e);
      std::process::exit(1);
    }
  }
}

fn run_repl() -> () {
  let rainbow = Rainbow::default();
  print!("Welcome to the ");
  rainbow.write("test-lang").expect("errored for some reason");
  println!(" REPL");

  let res: Result<Editor<()>, ReadlineError> = Editor::<()>::new();
  if res.is_err() { return; }

  let mut rl: Editor<()> = res.unwrap();
  loop {
    let readline_res: Result<String, ReadlineError> = rl.readline("➤ ");
    match readline_res {
      Ok(line) => {
        rl.add_history_entry(line.as_str());
        interpret(line.as_str());
      }
      Err(err) => {
        eprintln!("Exit: {:?}", err);
        break;
      }
    }
  }
}

pub fn run() -> () {
  let matches: ArgMatches = Command::new("lang")
    .arg(Arg::new("file_path")
      .help("The path of the file to compile")
      .required(false)
      .index(1))
    .get_matches();

  let path: Option<&String> = matches.get_one("file_path") as Option<&String>;
  if path.is_none() {
    run_repl();
  } else {
    let path: &String = path.unwrap();
    let file_contents: String = read_file(&path).unwrap();
    println!("{}", file_contents);
  }
}
