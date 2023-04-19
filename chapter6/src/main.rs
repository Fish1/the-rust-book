
fn main() {
    {
        let local_host = IpAddrStruct {
            kind: IpAddrKind::V4,
            data: String::from("127.0.0.1"),
        };

        let loop_back = IpAddrStruct {
            kind: IpAddrKind::V6,
            data: String::from("::1"),
        };

        dbg!(&local_host);
        dbg!(&loop_back);
    }
    {
        let local_host = IpAddrEnum::V4(String::from("127.0.0.1"));
        let local_host_2 = IpAddrEnum::V4_2(127, 0, 0, 1);
        let loop_back = IpAddrEnum::V6(String::from("::1"));


        dbg!(&local_host);
        dbg!(&local_host_2);
        dbg!(&loop_back);
    }
    {
        let quit = Message::Quit;
        let move_now = Message::Move { x: 5, y: 10 };
        let write = Message::Write(String::from("hello, world"));
        let change_color = Message::ChangeColor(5, 10, 20);

        quit.call();
        move_now.call();
        write.call();
        change_color.call();
    }
    {
        let some_number = Some(5);
        let some_number_2 = Some(6);
        let some_char = Some('e');

        println!("some number = {:?}", some_number);
    }
    {
        let c1 = Coin::Nickle;
        let c2 = Coin::Dime;
        let c3 = Coin::Quarter(UsState::NewYork);

        let value = get_coin_value(c1) + get_coin_value(c2) + get_coin_value(c3);
        println!("Nickle + Dime + Quarter = {}", value);
    }

    {
        let my_number = Some(5);

        println!("my_number = {:?}", my_number);

        let my_other_number = plus_two(my_number);

        println!("my_other_number = {:?}", my_other_number);

        let x = extract_value(my_other_number);
        let y = extract_value(None);

        println!("x = {}", x);
        println!("y = {}", y);
    }

    {
        let roll = 9;

        match roll {
            3 => println!("Double XP"),
            7 => println!("You Lose!"),
            other => println!("You rolled a {}", other)
        }

        match roll {
            3 => println!("Double XP"),
            7 => println!("You Lose!"),
            _ => println!("You didn't roll anything special")
        }

        match roll {
            3 => println!("Double XP"),
            7 => println!("You Lose!"),
            _ => ()
        }
    }

    {
        let my_thing = Some(32);
        let my_other: Option<i32> = None;

        if let Some(x) = my_thing {
            println!("The maximum is configured to be {x}");
        }

        // this wont match some, because my_other is None
        // therefor this will not run
        if let Some(x) = my_other {
            println!("This shouldn't work!!!");
        } else {
            println!("This will work instead!");
        }
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    data: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4_2(u32, u32 , u32, u32),
    V4(String),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Called Message Function!");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    NewYork,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn get_coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This quarter has the state of {:?}", state);
            25
        }
    }
}

fn extract_value(number: Option<i32>) -> i32 {
    match number {
        None => 0,
        Some(i) => i
    }
}

fn plus_two(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(i) => Some(i + 2)
    }
}