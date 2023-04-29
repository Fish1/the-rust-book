fn main() {
    use List::Cons;
    use List::Nil;

    println!("Hello, world!");

    let b = Box::new(5);
    println!("b = {}", b);

    let my_list = Cons(
        1, Box::new(
            Cons(
                2, Box::new(
                    Cons(3, Box::new(Nil))
                )
            )
        )
    );

    println!("my_list = {:?}", my_list);


    let x = 5;
    let y = Box::new(x);
    println!("x = {}", x);
    println!("y = {}", *y);

    let my_y = MyBox::new(x);
    println!("my_y = {}", *my_y);

    let point_x = Point {
        x: 3, y: 2
    };
    let point_y = Point {
        x: 1, y: 1
    };
    let point_z = point_x + point_y;
    println!("point_z = {:?}", point_z);
}

struct Cons {

}

// this is a recurssive definition
// rust cannot figure out how much memory to allocate for it
/*
enum List {
    Cons (
        i32,
        List,
    ),
    Nil
}
*/


// instead of storing a list inside the list,
// we can store a pointer to a list
// we can determine the size of a pointer
#[derive(Debug)]
enum List {
    Cons (
        i32,
        Box<List>
    ),
    Nil
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
