
use std::fs::File;

use std::io::{ErrorKind, Read};
use std::io;

use crate::errors::guess;

use super::guess::Guess;

pub fn open_file() {
  let file_result = File::open("hello.txt");

  let file = match file_result {
    Ok(file) => file,
    Err(error) => {
      match error.kind() {
        ErrorKind::NotFound => {
          match File::create("hello.txt") {
            Ok(file) => file,
            Err(error) => {
              panic!("Couldn't create file: {}", error)
            }
          }
        },
        other_error => {
          panic!("Other Errors! {}", other_error)
        }
      }
    }
  };
}

pub fn closure_file() {
  let file = File::open("hello2.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello2.txt").unwrap_or_else(|error| {
        panic!("Failed to create file: {}", error);
      })
    } else {
      panic!("Couldn't open file: {}", error);
    }
  });
}

pub fn expect() {
  // this will call the panic for us because it tries to get the file regardless
  // of it it failed or not
  // let example = File::open("hello3.txt").unwrap();

  // the expect will allow us to create our own panic
  // let example = File::open("hello3.txt").expect("Couldn't find hello3.txt");
}

pub fn propogate() -> Result<String, io::Error> {
  let file_result = File::open("hello4.txt");

  let mut file = match file_result {
    Ok(file) => file,
    Err(error) => return Err(error)
  };

  let mut username = String::new();
  match file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(error) => return Err(error)
  }
}

pub fn propogate_shorthand() -> Result<String, io::Error> {
  let mut file= File::open("hello5.txt")?;
  let mut username = String::new();
  file.read_to_string(&mut username)?;
  Ok(username)
}

pub fn propogate_test(index: usize) -> Option<i32> {
  let myvec = vec![1, 2, 3];
  let value = myvec.get(index)?;
  println!("no propogate");
  Some(value * 5)
}

pub fn testing_guess() {
  let myguess = Guess::new(34);
  println!("guess = {}", myguess.value());
  let myguess = Guess::new(500);
  println!("guess = {}", myguess.value());
}