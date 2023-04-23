pub fn create() {
  let mystr = String::new();
  println!("mystr = {}", mystr);

  let data = "initial data";
  let s = data.to_string();

  println!("s = {}", s);

  let mystr2 = String::from("initial");
  println!("mystr2 = {}", mystr2);
}

pub fn update() {
  let mut mystr = String::from("Hello,");
  mystr.push_str(" world!");

  println!("mystr = {}", mystr);

  let s2 = "more";
  mystr.push_str(s2);

  println!("s2 = {}", s2);

  println!("mystr = {}", mystr);
  mystr.push('L');
  println!("mystr = {}", mystr);
}

pub fn concat() {
  let s1 = String::from("AAAA");
  let s2 = String::from("BBBB");

  let s3 = s1 + &s2;

  // ownership of s1 has been moved to s3
  // println!("s1 = {}", s1);
  println!("s3 = {}", s3);


  let a = String::from("aaa");
  let b = String::from("bbb");
  let c = String::from("ccc");
  let x = format!("{a}-{b}-{c}");
  println!("x = {}", x);
}

pub fn iteration() {
  let s1 = "hello";

  for x in s1.chars() {
    println!("{}", x);
  }

  for x in s1.bytes() {
    println!("{}", x);
  }
}