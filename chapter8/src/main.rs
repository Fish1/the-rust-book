use std::hash;

use chapter8;

use chapter8::collections::vectors;
use chapter8::collections::strings;
use chapter8::collections::hashmaps;

fn main() {
    println!("Hello, world!");
    chapter8::run_test();
    vectors::create();
    vectors::access();
    vectors::iteration();
    vectors::enums();

    strings::create();
    strings::update();
    strings::concat();
    strings::iteration();

    hashmaps::create();
    hashmaps::ownership();
    hashmaps::usage();
}
