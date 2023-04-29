use add_one;
use add_two;

fn main() {
    let x = 3;
    let result = add_one::add_one(x);
    println!("x = {}, result = {}", x, result);

    let y = 3;
    let result = add_two::add_two(y);
    println!("y = {}, result = {}", y, result);
}
