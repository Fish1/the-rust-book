fn main() {
    let mut my_string = String::from("Hello, World!");
    my_string.push_str(" (this is pushed)");
    println!("my_string: {}", my_string);

    {
        let x = 5;
        let y = x;

        println!("x: {}, y: {}", x, y);
    }


    {
        let s1 = String::from("hello");

        // the s2 variable now owns the value of s1
        // s1 was moved to s2
        // the heap data is not copied, only the pointer to the data
        let s2 = s1;

        // we can now use s2
        println!("s2: {}", s2);

        // but we can't use s1 anymore
        // println!("s1: {}", s1);
    }

    {
        // this will perform a deep copy of the heap
        // data, which is expensive
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1: {}, s2: {}", s1, s2);
    }

    {
        // primitive types are copied, as they implement the Copy trait
        // this is because they are stored on the stack
        // and this is always a shallow copy
        let x = 5;
        let y = x;

        println!("x: {}, y: {}", x, y);
    }

    {
        let s1 = String::from("my string");
        takes_ownership(s1);
        // error because s1 was moved to the some_strings param of takes_ownership
        // takes_ownership(s1);

        let x = 5;
        makes_copy(x);
        // this is fine because x is a primitive type
        makes_copy(x);

        let s2 = String::from("my new string");
        let s3 = gives_ownership(s2);
        println!("s3: {}", s3);
        // the ownership of s2 was moved to s3
        // println!("s2: {}", s2);
    }

    {
        let s1 = String::from("my string");
        let (s2, len) = get_length(s1);
        println!("s2: {}, len: {}", s2, len);
    }

    {
        let s1 = String::from("my string");
        let len = get_length_from_ref(&s1);
        println!("s1: {}, len: {}", s1, len);
    }

    {
        let mut s1 = String::from("Hello");
        change(&mut s1);
        println!("s1: {}", s1);
    }

    {
        // can you have two mutable references to the same data?
        // no you can't
        // let mut s1 = String::from("Hello");
        // let r1 = &mut s1;
        // let r2 = &mut s1;
        // println!("r1: {}, r2: {}", r1, r2);

        let mut s1 = String::from("Hello");

        // we can have multiple immutable references at the same time
        let r1 = &s1;
        let r2 = &s1;
        // we cannot have a mutable reference while we have immutable references
        // let r3 = &mut s1;
        println!("r1: {}, r2: {}", r1, r2);

        // now that r1 and r2 are out of scope, we can have a mutable reference
        let r3 = &mut s1;
        println!("r3: {}", r3);
    }

    {
        let s1 = create();
        println!("s1: {}", s1);
    }

    {
        let s1 = String::from("hello world");
        let hello = &s1[0..5];
        let world = &s1[6..11];
        println!("hello: {}, world: {}", hello, world);
    }

    {
        let mut s1 = String::from("hello world");

        let w1 = first_word(&s1);

        // we can't do this because we have an immutable reference to s1
        // w2 is an immutable reference to a slice of s1
        // s1.push_str(" EDITED");

        println!("first word: {}", w1);
    }

    {
        // other slices
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..4];
        println!("a: {:?}", a);
        println!("slice: {:?}", slice);
    }
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
} // some_string goes out of scope and is dropped here

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
} // some_integer goes out of scope and is dropped here, but it's a clone so nothing happens

fn gives_ownership(some_string: String) -> String {
    println!("some_string: {}", some_string);
    some_string
} // some_string goes out of scope and is dropped here

fn get_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
} // we must return the string here, because it was moved to the function parameter

fn get_length_from_ref(some_string: &String) -> usize {
    some_string.len()
} // some_string is dropped, but not the string it is refering too

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn create() -> String {
    let s = String::from("this string was created inside a function");
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}