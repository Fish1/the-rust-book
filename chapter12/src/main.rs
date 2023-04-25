use std::{env, process};
use chapter12::run;
use chapter12::Config;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {err}");
    process::exit(1)
  });

  if let Err(e) = run(config) {
    println!("Problem reading file: {e}");
    process::exit(1);
  }
}

