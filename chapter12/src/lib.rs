use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.path)?;
  println!("Text: {content}");
  Ok(())
}

pub struct Config {
  query: String,
  path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    let query_option = args.get(1);
    let query = match query_option {
      Some(value) => value.clone(),
      None => return Err("No query provided")
    };
    let path_option = args.get(2);
    let path = match path_option {
      Some(value) => value.clone(),
      None => return Err("No path provided")
    };
    Ok(Config { query, path })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    )
  }
}