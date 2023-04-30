fn main() {
    use List::Cons;
    use List::Nil;
    use std::rc::Rc;

    println!("Hello, world!");

    let b = Box::new(5);
    println!("b = {}", b);

    let my_list = Rc::new(Cons(
        1, Rc::new(
            Cons(
                2, Rc::new(
                    Cons(3, Rc::new(Nil))
                )
            )
        )
    ));

    println!("my_list = {:?}", my_list);

    let my_list_2 = Cons(3, Rc::clone(&my_list));
    println!("my_list count = {}", Rc::strong_count(&my_list));
    let my_list_3 = Cons(4, Rc::clone(&my_list));
    println!("my_list count = {}", Rc::strong_count(&my_list));

    println!("my_list_2 = {:?}", my_list_2);
    println!("my_list_3 = {:?}", my_list_3);


    {
        let my_list_4 = Cons(5, Rc::clone(&my_list));
        println!("my_list count = {}", Rc::strong_count(&my_list));
    }
    println!("my_list count = {}", Rc::strong_count(&my_list));

    let x = 5;
    let y = Box::new(x);
    println!("x = {}", x);
    println!("y = {}", *y);

    let my_y = MyBox::new(x);
    my_coersion_test(&my_y);

    let mut my_smart_pointer = MySmartPointer {
        data: 123.456,
    };

    let my_smart_pointer_2 = MySmartPointer {
        data: 789.123,
    };

    drop(my_smart_pointer);

    println!("Early drop...");

    let point_x = Point {
        x: 3, y: 2
    };
    let point_y = Point {
        x: 1, y: 1
    };
    let point_z = point_x + point_y;
    println!("point_z = {:?}", point_z);

    let x = Rc::new(55);
    let z = Rc::clone(&x);
    let y = RefCell<i32>(5);

    println!("x = {}", x);
    println!("z = {}", z);
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
        std::rc::Rc<List>
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

struct MySmartPointer {
    data: f64
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropped!!! {}", self.data);
    }
}

fn my_coersion_test(x: &i32) {
    println!("you passed an i32 of {x}");
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
