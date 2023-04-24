
// generic lifetime 'a will be equal to whichever lifetime is smaller,
// a or b's lifetime
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
  if a.len() > b.len() {
    a
  } else {
    b
  }
}

// set the lifetime of the return to that of the left parameter
fn longest_left<'a>(a: &'a str, b: &str) -> &'a str {
  a
}

// set the lifetime of the return to that of the right parameter
fn longest_right<'a>(a: &str, b: &'a str) -> &'a str {
  b
}

struct TestThing<'a> {
  part: &'a str
}

// we don't need to use explicit lifetimes with this function,
// because the lifetimes of the reference can be infered
fn first_word<'a>(s: &'a str) -> &'a str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}

struct Vector<'a> {
  x: &'a i32,
  y: &'a i32,
}

impl<'a> Vector<'a> {
  fn add(&self) -> i32 {
    self.x + self.y
  }
}

pub fn lifetime() {
  let s1 = String::from("abcd");
  let result;
  let result_left;
  let result_right;
  {
    let s2 = String::from("abc");
    result = longest(&s1, &s2);
    result_left = longest_left(&s1, &s2);
    result_right = longest_right(&s1, &s2);
    println!("longest = {}", result);
    println!("longest_left = {}", result_left);
    println!("longest_right = {}", result_right);
  }
  // we cannot access result here
  // because results lifetime is set to the shorter lifetime of s2
  // println!("longest = {}", result);

  // we cann access result_left
  // because we told the function to set the lifetime of the
  // result to the left parameter, which is s1
  println!("longest_left = {}", result_left);

  // we cannot access result_right
  // because the function sets the life time to that of
  // s2. Therefor it goes out of scope
  // println!("longest_right = {}", result_right);


  let novel = String::from("My Novel");
  let x = TestThing {
    part: &novel
  };
  println!("x.part = {:?}", x.part);

  let my_sentence = String::from("hello world");
  let first_word = first_word(&my_sentence);
  println!("First Word = {}", first_word);

  let x = 5;
  {
    let y = 6;
    let my_vector = Vector {
      x: &x,
      y: &y
    };
    
  }

  println!("my_vector add = {}", x);

}