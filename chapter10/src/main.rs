use chapter10::generics::testing;
use chapter10::generics::structs;
use chapter10::generics::enums;

use chapter10::traits::basics;
use chapter10::traits::lifetimes;

fn main() {
    testing::largest_stuff();
    structs::do_point_stuff();
    enums::do_stuff();

    basics::do_basics();
    lifetimes::lifetime();
}
