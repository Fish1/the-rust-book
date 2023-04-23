
pub fn create() {
  // how to create vectors
  let mut v = Vec::new();

  v.push(3);
  v.push(5);
  v.push(8);

  let mut v1 = vec![1, 2, 3];

  v1.push(1);
  v1.push(2);
  v1.push(3);

  // the get function will return an option
  // None if there is no value at the index
  // otherwise a Some
  let third = v1.get(2);
  match third {
    None => println!("No third element!"),
    Some(value) => println!("third: {}", value)
  }

  let huge = v1.get(300);
  match huge {
    None => println!("No 300th element!"),
    Some(value) => println!("300th: {}", value)
  }

  // this will panic if there is no fourth element
  let fourth = v1[3];
  println!("fourth: {}", fourth);

}

pub fn access() {
  let mut v1 = vec![1,2,3,4];

  let x = &v1[1];

  // cannot push here because x is immutable
  // v1.push(7);

  println!("x: {}", x);
}

pub fn iteration() {
  let mut v1 = vec![3,5,2,6];

  for i in &v1 {
    println!("{}", i);
  }

  for i in &mut v1 {
    *i *= 2;
  }

  println!("{:?}", v1);
}

pub fn enums() {
  enum Cell {
    Int(i32),
    Float(f32),
    Text(String),
  }
  let v1 = vec![
    Cell::Int(3),
    Cell::Float(3.45),
    Cell::Text(String::from("Hello World")),
  ];

  for c in &v1 {
    match c {
      Cell::Int(value) => println!("{}", value),
      Cell::Float(value) => println!("{}", value),
      Cell::Text(value) => println!("{}", value)
    }
  }

  enum MyThing {
    Data1(String),
    Data2(i32),
  }

  let v2 = vec![
    MyThing::Data1(String::from("Hellooo")),
    MyThing::Data2(2345)
  ];

  for x in v2 {
    match x {
      MyThing::Data1(_value) => println!("This is a string"),
      MyThing::Data2(_value) => println!("This is a number")
    }
  }
}
