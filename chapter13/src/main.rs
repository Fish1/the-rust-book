use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Blue,
        ]
    };

    store.apply_thing(|shirt| println!("this is {:?}", shirt));

    let user1 = Some(ShirtColor::Red);
    let shirt1 = store.giveaway(user1, || ShirtColor::Blue);

    println!("User1 Got: {:?}", shirt1);

    let user2 = None;
    let shirt2 = store.giveaway(user2, || ShirtColor::Blue);


    println!("User2 Got: {:?}", shirt2);

    let user3 = Some(ShirtColor::Blue);
    let shirt3 = store.giveaway(user3, || ShirtColor::Red);

    println!("User3 Got: {:?}", shirt3);

    let my_closure = |x: u32| -> u32 {
        x + 5
    };
    println!("my_closure(5) = {}", my_closure(5));

    let my_other_closure = |x| x + 3;
    println!("my_other_closure(2) = {}", my_other_closure(3));


    let my_vec = vec![1, 2, 3];
    let only_borrow = |x| println!("x = {:?}", x);
    only_borrow(my_vec);

    let mut my_vec2 = vec![1, 2, 3];
    let mut borrows_mutably = || my_vec2.push(5);
    // we cannot print because the borrows_mutable closure has borrowed
    // it mutably
    // println!("{:?}", my_vec2);
    borrows_mutably();
    println!("{:?}", my_vec2);


    let my_vec3 = vec![1, 5, 9];
    println!("before = {:?}", my_vec3);
    thread::spawn(move || {
        println!("thread: {:?}", my_vec3);
        my_vec3
    }).join().unwrap();

    // we can no longer use my_vec3
    // because it was moved into the closure of the thread
    // this guarentees we cannot access data being manipulated
    // in a multithreaded application
    // println!("after: {:?}", my_vec3);
    
    let mut list = [
        Rectangle { width: 10, height: 9 },
        Rectangle { width: 13, height: 4 },
        Rectangle { width: 8, height: 10 },
    ];

    let mut sort_operations: Vec<String> = Vec::new();
    let my_string = String::from("move me LOL!");

    let mut sort_operations = 0;
    println!("list before = {:?}", list);
    list.sort_by_key(|rectangle| {
        // this is a FnMut that gets called multiple times
        // therefor the following line will not work
        // as it tries to take the ownership from
        // my_string and give it to sort_operations multiple times
        // you cannot move ownership from something that doesn't
        // have ownership
        // sort_operations.push(my_string);
        
        // we can take hold of the sort_operations mut numbers
        // as this is a one time thing, and increment
        sort_operations += 1;
        rectangle.width
    });
    println!("list after = {:?}", list);
    println!("sort_operations = {}", sort_operations);


    let mut my_list = vec![1, 3, 2, 5];
    let mut my_iter = my_list.iter();
    let x = my_iter.next().unwrap();
    println!("x = {}", x);

    let my_new_iter = my_list.iter();
    let k = &my_new_iter;
    // we cannot loop over iter because k borrowed my_new_iter
    // behind the scenes the loop will try to take ownership
    /*
    for iter in my_new_iter {
        println!("iter = {}", iter);
    }
    */
    println!("k = {:?}", k);

    let my_mut_iter = my_list.iter_mut();
    for item in my_mut_iter {
        *item = *item + 1;
    }

    let my_sum_iter = my_list.iter();
    let sum: i32 = my_sum_iter.sum();
    println!("sum = {}", sum);

    let my_map_iter = my_list.iter();
    let new_thing: Vec<i32> = my_map_iter.map(|x| x - 2).collect();
    println!("new_thing = {:?}", new_thing);

    let my_owned_iter = my_list.into_iter();
    for item in my_owned_iter {
        println!("item = {}", item);
    }
    // the data of my_list was moved into my_owned_iter
    // therefor we cannot use my_list anymore
    // println!("my_list = {:?}", my_list);
    
    let shoes = vec![
        Shoe { size: 1, style: String::from("sneaker") },
        Shoe { size: 2, style: String::from("sneaker") },
        Shoe { size: 3, style: String::from("dress") },
        Shoe { size: 4, style: String::from("moc") },
    ];

    println!("shoes = {:?}", shoes);

    let shoes_iterator = shoes.iter();
    let wanted_style = String::from("sneaker");
    let filtered_shoes: Vec<&Shoe> = shoes_iterator.filter(|shoe| shoe.style == wanted_style).collect();
    println!("filtered shoes = {:?}", filtered_shoes);
    println!("shoes = {:?}", shoes);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn has_most_of(&self) -> ShirtColor {
        let mut blue_count = 0;
        let mut red_count = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1
            }
        }
        if blue_count > red_count {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }

    fn has_in_stock(&self, color: ShirtColor) -> bool {
        for shirt in &self.shirts {
            if *shirt == color {
                return true;
            }
        }
        false
    }

    fn giveaway<OnNoShirt>(&self, preference: Option<ShirtColor>, on_no_shirt: OnNoShirt) -> ShirtColor 
        where
            OnNoShirt: FnOnce() -> ShirtColor
    {
        let to_remove = preference.unwrap_or_else(|| self.has_most_of());
        if self.has_in_stock(to_remove) {
            to_remove
        } else {
            on_no_shirt()
        }
    }

    fn apply_thing<OnItem>(&self, mut on_item: OnItem)
        where
            OnItem: FnMut(&ShirtColor) -> ()
    {
        for shirt in &self.shirts {
            on_item(shirt);
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}
