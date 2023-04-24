enum MyOption<Type> {
  Some(Type),
  None,
}

pub fn do_stuff() {
  let x = MyOption::Some(5);
  let y = MyOption::Some('x');

  match x {
    MyOption::Some(v) => println!("v = {}", v),
    MyOption::None => println!("none...")
  }
  
  match y {
    MyOption::Some(v) => println!("v = {}", v),
    MyOption::None => println!("none...")
  }
}