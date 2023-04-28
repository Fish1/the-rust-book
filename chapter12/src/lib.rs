use std::{fs, error::Error, env::{self}};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.path)?;
  let matches = match config.ignore_case {
    true => search_case_insensative(&config.query, &contents),
    false => search_case_sensative(&config.query, &contents),
  };
  for line in matches {
    println!("{line}");
  }
  Ok(())
}

pub struct Config {
  query: String,
  path: String,
  ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query provided")
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("no path provided")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, path, ignore_case })
    }
}

pub fn search_case_sensative<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
    .filter(
        |line| line.contains(&query)
    )
    .collect()
}

pub fn search_case_insensative<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
    .filter(
        |line| line.to_lowercase().contains(&query)
    )
    .collect()
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
Pick three.
Duct tape";
    assert_eq!(
      vec!["safe, fast, productive."],
      search_case_sensative(query, contents)
    )
  }

  #[test]
  fn case_insensitive() {
    let query = "aaa";
    let contents = "\
AAAA
bbbb
bbAAA";
    assert_eq!(
      vec!["AAAA", "bbAAA"],
      search_case_insensative(query, contents)
    )
  }
}
