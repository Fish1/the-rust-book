
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user2 = build_user(String::from("joe"), String::from("joe@gmail.com"));
    print_user(&user2);

    println!("------------------");

    let user3 = User {
        active: false,
        ..user2
    };
    print_user(&user3);

    // we cannot use user2 anymore
    // because email and username are moved to user3
    // as they are not copied
    // print_user(&user2);

    println!("------------------");

    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("black: {}, {}, {}", black.0, black.1, black.2);
        println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
    }

    println!("------------------");

    {
        let width = 30;
        let height = 50;
        println!("The area is {} units squared", calc_area(width, height));

        let rect1 = (30, 50);
        println!("The area is {} units squared", calc_area_tuple(rect1));

        let rect2 = Rectangle {
            width: dbg!(30 * 2),
            height: 50,
        };

        dbg!(&rect2);

        let rect3 = Rectangle {
            width: 10,
            height: 40,
        };

        println!("rect2: The area is {} units squared", rect2.area());
        println!("rect3: The area is {} units squared", rect3.area());

        println!("rect2 can hold rect3: {}", rect2.can_hold(&rect3));
        println!("rect3 can hold rect2: {}", rect3.can_hold(&rect2));
    }

    {
        let the_square = Rectangle::square(10);
        let the_square_2 = Rectangle::square(20);

        dbg!(&the_square);
        dbg!(&the_square_2);

        println!("the_square: The area is {} unites squared", the_square.area());
        println!("the_square_2: The area is {} unites squared", the_square_2.area());
    }
}

fn calc_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calc_area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn calc_area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn print_user(user: &User) {
    println!("active: {:?}", user);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        sign_in_count: 0,
        username,
        email,
    }
}
