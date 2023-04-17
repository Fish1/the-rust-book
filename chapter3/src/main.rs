fn main() {
    let x = 5;

    println!("The value of x is: {}", x);

    let x = 6;

    println!("The shadow value of x is: {}", x);

    if x == 6 {
        println!("x is 6");
    } else {
        println!("x is not 6");
    }

    let mut y = if x == 6 { 10 } else { 15 };

    println!("The value of y is: {}", y);

    y = 7;

    println!("The value of y is: {}", y);

    println!("return: {}", my_function(y));

    if y == 6 {
        println!("y is 6");
    } else if y == 7 {
        println!("y is 7");
    } else {
        println!("y is not 6 or 7");
    }

    let mut z = 0;
    let z_rez = loop {
        z += 1;
        println!("z = {}", z);
        if z == 5 {
            break z * 2;
        }
    };
    println!("The value of z_res is: {}", z_rez);


    'first_loop: loop {
        println!("first loop");
        let mut x = 0;
        loop {
            x += 1;
            println!("second loop");
            if x == 3 {
                break 'first_loop;
            }
        }
    }

    let mut counter = 0;
    while counter < 3 {
        println!("counter = {}", counter);
        counter += 1;
    }

    let array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("array[{}] = {}", index, array[index]);
        index += 1;
    }
    for element in array {
        println!("element = {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

fn my_function(x: i32) -> i32 {
    println!("Hello, world! {}", x);
    return x + 3;
}
