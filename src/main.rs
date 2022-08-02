use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader}; // prelude::* is needed for lines() to work
use std::env;
use std::path::PathBuf;
use shellexpand::tilde;

use regex::Regex;

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

  //let mut home_dir_path = tilde("~/.config/define/gcide.dict");
  let home_dir_path = "/home/vr0n/.config/define/gcide.dict";
  let dict_path = PathBuf::from(home_dir_path);
  let dict = File::open(fs::canonicalize(&dict_path)?).expect("Could not open file");
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

  Ok(())
}
