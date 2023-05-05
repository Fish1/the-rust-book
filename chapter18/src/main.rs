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


    // we use pattern matching in function parameters
    fn my_func(x: i32, (w, z): (i32, f64)) {
        println!("x = {x}, (w, z) = ({w},{z})");
    }
    my_func(5, (7, 7.5));


    let x = Some(55);
    // this doesn't work becuase it's refutable
    // let Some(y) = x;
    //
    // this does work because we handle the case
    // when it gets refuted
    if let Some(y) = x {
        println!("y = {}", y);
    } else {
        println!("REFUTED!!!");
    }

    // not needed because z = 5 will always work
    if let z = 5 {
        println!("z = {z}");
    }


    let my_pattern = 3;
    match my_pattern {
        // 1 gets refuted because it doesn't match 3
        1 => println!("ONE"),
        // 2 gets refuted because it doesn't match 3
        2 => println!("TWO"),
        // 3 matches 3 !!!
        3 => println!("THREE"),
        4 => println!("FOUR"),
        _ => println!("OTHER")
    }


    let pat = Some(5);
    match pat {
        // Some(5) doesn't match Some(50)
        Some(50) => println!("Some 50!"),
        // Some(5) does match Some(y) where y is any i32
        Some(y) => println!("Some Value! {y}"),
        _ => println!("Other cases...")
    }

    let mypat = Some(25);
    match mypat {
        Some(5) => println!("5"),
        // matches Some(20) or Some(25)
        Some(20) | Some(25) => println!("Some 20 or 25!"),
        _ => println!("Default match!")
    }

    let number = 7;
    match number {
        1 => println!("1"),
        2 => println!("2"),
        // matches between 3 and 20
        3..=20 => println!("between 3 and 20"),
        _ => println!("default match")
    }


    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point {
        x: 0, y: 7
    };
    // destructure a struct by using patterns
    // x matches x, y matches y
    let Point { x: a, y: b } = p;
    println!("a = {a}, b = {b}");

    // shorthand for the above pattern
    let Point { x, y } = p;
    println!("x = {x}, y = {y}");


    let p = Point {
        x: 0, y: 7
    };

    match p {
        // matches any x and y = 0
        Point { x, y: 0 } => println!("on the y axis"),
        // matches any y and x = 0
        Point { x: 0, y } => println!("on the x axis"),
        // matches any x and y
        Point { x, y } => println!("Point = {x}, {y}")
    }


    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        ChangeColor(Color),
        Write(String)
    }

    let msg = Message::ChangeColor(Color::Hsv(4, 6, 9));
    match msg {
        Message::Quit => println!("quit match!"),
        Message::ChangeColor(Color::Rgb(r,g,b)) => println!("RGB {r} {g} {b}"),
        Message::ChangeColor(Color::Hsv(h,s,v)) => println!("HSV {h} {s} {v}"),
        Message::Write(str) => println!("write match, {str}"),
    }

    let ((feet, inches), Point {x, y}) = ((10, 20), Point { x: 7, y: 10 });
    println!("feet = {feet}, inches = {inches}");
    println!("x = {x}, y = {y}");

    // we can ignore matches with the _
    let (_, y) = (1, 2);
    println!("y = {y}");
    
    fn do_stuff(_: i32, y: i32) {
        println!("y = {y}");
    }
    do_stuff(3, 5);

    let setting = Some(5);
    let new_setting = Some(10);
    match (setting, new_setting) {
        (Some(_), Some(_)) => {
            println!("cannot have two Somes")
        },
        _ => {
            println!("Settings changed!")
        }
    }

    let s = Some(String::from("my thing"));
    if let Some(_x) = s {
        println!("found a string!!!");
    }
    // we cannot use s here becuase it was
    // moved into Some(_x)
    // println!("s = {:?}", s);
    
    let s2 = Some(String::from("my thing 2"));
    if let Some(_) = s2 {
        println!("found a string!!!");
    }
    // we can use s2 here because we never bound it
    // to another variable
    println!("s2 = {:?}", s2);


    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
        w: i32
    }

    // we can use .. to ignore the remaing of the match
    let p = Point3D { x: 5, y: 7, z: 5, w: 7 };
    match p {
        Point3D { x, z, .. } => {
            println!("x = {x}, z = {z}")
        }
    }

    let list = (1,2,3,4,5);
    match list {
        (a,b,..,e) => {
            println!("{a} {b} {e}");
        }
    }

    let num = Some(4);
    match num {
        Some(x) if x != 4 => println!("X is NOT 4"),
        Some(_) => println!("X is 4"),
        _ => ()
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no")
    }

    enum Message2 {
        Hello { id: i32 }
    }

    let msg = Message2::Hello {
        id: 5
    };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7 ,
        } => {
            println!("id = {id_variable}")
        },
        Message2::Hello {
            id: 10..=12
        } => {
            println!("found in another range");
        },
        _ => ()
    }
}
