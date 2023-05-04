pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {

    pub fn new(list: Vec<Box<dyn Draw>>) -> Screen {
        Screen {
            components: list
        }
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button... width {}, height {}, label {:?}",
            self.width, self.height, self.label
        )
    }
}

pub struct Picture {
    pub uri: String,
    pub quality: f64,
}

impl Draw for Picture {
    fn draw(&self) {
        println!("drawing picture... uri {}, quality {}",
            self.uri, self.quality
        )
    }
}
