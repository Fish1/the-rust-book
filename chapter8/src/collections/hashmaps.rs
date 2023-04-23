
use std::collections::HashMap;

pub fn create() {
  let mut scores = HashMap::new();

  scores.insert(String::from("blue"), 5);
  scores.insert(String::from("yello"), 10);

  let t1 = String::from("blue");

  let mut score = scores.get(&t1);
  match score {
    Some(value) => println!("blue: {}", value),
    None => println!("No Score!")
  }

  let teamname = String::from("yello");
  let yscore = scores.get(&teamname).copied().unwrap_or(0);
  println!("Yello: {}", yscore);
  
  let teamname = String::from("green");
  let yscore = scores.get(&teamname).copied().unwrap_or(0);
  println!("Green: {}", yscore);


  for (key, value) in &scores {
    println!("{key} = {value}");
  }
}

pub fn ownership() {
  let x = String::from("Hello");
  let y = String::from("world");

  let mut map = HashMap::new();
  map.insert(x, y);

  // ownership of the data in x and y has been given to map
  // println!("x = {}", x);
  // println!("y = {}", y);
}

pub fn usage() {
  // update
  let mut map = HashMap::new();

  map.insert("x", 1);
  println!("map = {:?}", map);
  map.insert("x", 2);
  println!("map = {:?}", map);

  map.entry("y").or_insert(7);
  map.entry("x").or_insert(3);
  println!("map = {:?}", map);

  let val = map.entry("x").or_insert(0);
  *val *= 5;
  println!("map = {:?}", map);
}
