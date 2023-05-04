fn main() {
    // we can use patterns in rust with many different syntaxes
    
    let x = Some(5);
    match x {
        Some(v) => println!("v = {}", v),
        None => ()
    }

    let color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = color {
        // this runs if color has a Some and not a None
    } else if is_tuesday {
        // this runs if tuesday is true
    } else if let Ok(age) = age {
        if age > 30 {
            println!("YOURE OLDER THAN 30");
        } else {
            println!("YOURE NOT OLDER THAN 30");
        }
    } else {
        println!("oopsss...");
    }


    let mut stack = Vec::new();
    stack.push(8);
    stack.push(9);
    stack.push(10);

    // loop through the while loop if pop keeps matching to a Some
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!['a', 'b', 'c', 'd'];

    // we can use a pattern to destructure a tuple in a for loop
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}!");
    }


    // let PATTERN = EXPRESSION;
    let x = 5;
    // here we match a tuple to a pattern
    // pattern (x, y, z)
    let (x, y, z) = (1, 2, 3);


    fn my_func(x: i32, (w, z): (i32, f64)) {
        println!("x = {x}, (w, z) = ({w},{z})");
    }

    my_func(5, (7, 7.5));
}
