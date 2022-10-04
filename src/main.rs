use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader}; // prelude::* is needed for lines() to work
use std::path::PathBuf;

use regex::RegexBuilder;
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

  let rx = format!("^{} ", &args[1]);
  let word_match = RegexBuilder::new(&rx).case_insensitive(true).build()?;

  let dict_path = PathBuf::from(tilde("~/.config/define/gcide.dict").as_ref()); // cow as ref
  let dict = File::open(dict_path).expect("Could not open file");
  let contents = BufReader::new(dict);

  let mut found = false;
  for line in contents.lines() {
    let line_result = &line?;

    if found {
      if !(line_result == "") {
        println!("{}", line_result);
      } else {
        std::process::exit(0);
      }
    }

    if word_match.is_match(&line_result) {
      logo();
      println!("{}", line_result);
      found = true;
    }
  }

  println!("{} not found in dictionary", &args[1]);

  Ok(())
}
