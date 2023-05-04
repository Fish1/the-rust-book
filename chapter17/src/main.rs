use std::time::Duration;
use std::thread;

use chapter17::{AveragedCollection, gui::{Screen, Button, Picture}, mystate::{Enemy, EnemyManager}, better_post::Post2};

fn main() {
    println!("Hello, world!");
    
    let mut avgc = AveragedCollection::new();
    avgc.add(1);
    avgc.add(9);
    avgc.add(1);
    avgc.add(3);

    println!("avgc average = {}", avgc.average());

    let mut screen = Screen::new(vec![
        Box::new(Button {
            width: 5,
            height: 7,
            label: String::from("cool")
        })
    ]);

    let btn = Box::new(Button {
        width: 1,
        height: 1,
        label: String::from("small")
    });
    screen.components.push(btn);

    let picture = Box::new(Picture {
        uri: String::from("https://google.com"),
        quality: 0.8,
    });
    screen.components.push(picture);

    screen.run();

    let enemy1 = Box::new(Enemy::new("bob"));
    let enemy2 = Box::new(Enemy::new("jacob"));

    let mut enemyManager = EnemyManager::new();
    enemyManager.add(enemy1);
    enemyManager.add(enemy2);

    /*
    loop {
        enemyManager.run();
        thread::sleep(Duration::from_millis(500));
    }
    */
    
    let mut post = Post2::new();
    post.add_text("this is cool");
    let post = post.request_review();
    let post = post.approve();
    println!("p content = {}", post.content());

}
