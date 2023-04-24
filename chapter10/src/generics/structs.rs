
struct Point<Type> {
  x: Type,
  y: Type,
}

impl Point<i32> {
  fn print_i32(&self) {
    println!("i32 x = {}, y = {}", self.x, self.y);
  }
}

impl Point<char> {
  fn print_char(&self) {
    println!("char x = {}, y = {}", self.x, self.y);
  }
}

impl<Type: std::fmt::Display> Point<Type> {
  fn print_generic(&self) {
    println!("generic x = {}, y = {}", self.x, self.y);
  }
}

struct PointOther<Type1, Type2> {
  x: Type1,
  y: Type2,
}

pub fn do_point_stuff() {
  let point1 = Point {
    x: 'a',
    y: 'b'
  };

  println!("point1 = x: {}, y: {}", point1.x, point1.y);
  point1.print_char();
  point1.print_generic();

  let point2 = Point {
    x: 1,
    y: 2,
  };

  println!("point2 = x: {}, y: {}", point2.x, point2.y);
  point2.print_i32();
  point2.print_generic();

  let point3 = PointOther {
    x: 'a',
    y: 1,
  };

  println!("point3 = x: {}, y: {}", point3.x, point3.y);
}