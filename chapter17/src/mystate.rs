use std::{thread::{JoinHandle, self}, time::Duration};


pub struct Enemy {
    pub name: String
}

impl Enemy {
    pub fn new(name: &str) -> Enemy {
        Enemy {
            name: String::from(name)
        }
    }
}

trait Wander {
    fn move_randomly(&self);
}

trait Attack {
    fn swing_sword(&self);
}

trait RunAway {
    fn turn_around(&self);
}

impl Wander for Enemy {
    fn move_randomly(&self) {
        println!("moving randomly...");
    }
}

impl Attack for Enemy {
    fn swing_sword(&self) {
        println!("swinging sword...");
    }
}

impl RunAway for Enemy {
    fn turn_around(&self) {
        println!("turning around...");
    }
}

pub struct EnemyManager {
    t1: Box<Option<JoinHandle<()>>>,
    t2: Box<Option<JoinHandle<()>>>,
    t3: Box<Option<JoinHandle<()>>>,
    wandering_enemies: Vec<Box<dyn Wander>>,
    attacking_enemies: Vec<Box<dyn Attack>>,
    running_enemies: Vec<Box<dyn RunAway>>,
}

impl EnemyManager {

    pub fn new() -> EnemyManager {
        let mut em = EnemyManager {
            t1: Box::new(None),
            t2: Box::new(None),
            t3: Box::new(None),
            wandering_enemies: vec![],
            attacking_enemies: vec![],
            running_enemies: vec![],
        };


        em.t1 = Box::new(Some(thread::spawn(|| {
            loop {
                println!("t1");
                thread::sleep(Duration::from_millis(500));
            }
        })));
        
        em.t2 = Box::new(Some(thread::spawn(|| {
            loop {
                println!("t2");
                thread::sleep(Duration::from_millis(500));
            }
        })));
        
        em.t3 = Box::new(Some(thread::spawn(|| {
            loop {
                println!("t3");
                thread::sleep(Duration::from_millis(500));
            }
        })));

        em
    }

    pub fn add(&mut self, enemy: Box<Enemy>) {
        self.wandering_enemies.push(enemy);
    }

    pub fn run(&self) {
        for enemy in &self.wandering_enemies {
            enemy.move_randomly();
        }
        for enemy in &self.attacking_enemies {
            enemy.swing_sword();
        }
        for enemy in &self.running_enemies {
            enemy.turn_around();
        }
    }
}


