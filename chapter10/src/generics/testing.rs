
fn get_largest_i32(vec: &[i32]) -> &i32 {
  let mut largest = &vec[0];

  for number in vec {
    if number > largest {
      largest = number;
    }
  }

  largest
}

fn get_largest_char(vec: &[char]) -> &char {
  let mut largest = &vec[0];

  for char in vec {
    if char > largest {
      largest = char;
    }
  }

  largest
}

fn get_largest<Type: std::cmp::PartialOrd>(vec: &[Type]) -> &Type {
  let mut largest = &vec[0];

  for item in vec {
    if item > largest {
      largest = item;
    }
  }

  largest
}

pub fn largest_stuff() {
  let myvec = vec![1, 5, 2, 9, 3, 10, 33, 2, 1];
  let largest1 = get_largest_i32(&myvec);
  println!("largest1 = {}", largest1);

  let myvec2 = vec![5, 9, 3];
  let largest2 = get_largest_i32(&myvec2);
  println!("largest2 = {}", largest2);

  let myvec3 = vec!['a', 'y', 'm', 'b'];
  let largest3 = get_largest_char(&myvec3);
  println!("largest3 = {}", largest3);

  let myvec4 = vec!['a', 'm', 'b', 'c'];
  let myvec5 = vec![9, 3, 4, 10, 7];
  let largest4 = get_largest(&myvec4);
  let largest5 = get_largest(&myvec5);
  println!("largest4 = {}", largest4);
  println!("largest5 = {}", largest5);
}