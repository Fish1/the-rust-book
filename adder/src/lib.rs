#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn it_no_works() {
        let result = 2 + 5;
        assert_eq!(result, 4);
    }
    #[test]
    fn rectangle_test() {
        let bigger = Rectangle {
            width: 5,
            height: 5,
        };
        let smaller = Rectangle {
            width: 4,
            height: 4,
        };
        assert_eq!(bigger.can_hold(&smaller), true);
    }
    #[test]
    fn rectangle_test_fail() {
        let bigger = Rectangle {
            width: 5,
            height: 6,
        };
        let smaller = Rectangle {
            width: 10,
            height: 4,
        };
        assert_eq!(bigger.can_hold(&smaller), true);
    }
}
