use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader}; // prelude::* is needed for lines() to work
use std::path::PathBuf;

use regex::Regex;
use shellexpand::tilde;

fn logo() {
  println!("╔═══╗     ╔═╗          ");
  println!("╚╗╔╗║     ║╔╝          ");
  println!(" ║║║║╔══╗╔╝╚╗╔╗╔═╗ ╔══╗");
  println!(" ║║║║║╔╗║╚╗╔╝╠╣║╔╗╗║╔╗║");
  println!("╔╝╚╝║║║═╣ ║║ ║║║║║║║║═╣");
  println!("╚═══╝╚══╝ ╚╝ ╚╝╚╝╚╝╚══╝");
}

fn help(func: &str) {
  logo();
  println!("\nUsage: {} <word to define>", func);
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    help(&args[0]);
    std::process::exit(1);
  }

  let word = &args[1].to_lowercase();
  let pre = "^(";
  let post = ") ";
  let rx = format!("{}{}{}", pre, word, post);

  let line_start = Regex::new("^[a-zA-Z]").unwrap();
  let word_match = Regex::new(&rx).unwrap();

  let dict_cow = tilde("~/.config/define/gcide.dict");
  let dict_path = PathBuf::from(dict_cow.as_ref()); // must explicitly deref cow
  let dict = File::open(dict_path).expect("Could not open file");
  let contents = BufReader::new(dict);

  let mut found = false;
  logo();
  for line in contents.lines() {
    let line_result = &line?;

    if found {
      if !(line_result == "") {
        println!("{}", line_result);
      } else {
        std::process::exit(0);
      }
    }

    if line_start.is_match(line_result) {
      if word_match.is_match(&line_result.to_lowercase()) {
        println!("{}", line_result);
        found = true;
      }
    }
  }

  println!("{} not found in dictionary", &args[1]);

  Ok(())
}
