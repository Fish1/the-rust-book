pub mod gui;
pub mod mystate;
pub mod post;
pub mod better_post;

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {

    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.calculate_average();
        self.list.push(value);
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.calculate_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn calculate_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}
