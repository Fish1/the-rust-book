struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    println!("I am adding {} and {}", left, right);
    left + right
}

pub fn add_two(var: i32) -> i32 {
    var + 2
}

pub fn less_than_100(var: i32) -> i32 {
    if var >= 100 {
        panic!("the number is too big");
    }
    var
}

pub fn get_secret_number(index: usize) -> Result<i32, String> {
    let myvec = vec![1, 3, 4, 6, 6];
    match myvec.get(index) {
        Some(value) => Ok(*value),
        None => Err(String::from("The value doesn't exist!!!"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn it_should_not_work() {
        assert_eq!(add(2, 5), 4)
    }

    #[test]
    #[should_panic(expected = "the number is too big")]
    fn expect_panic() {
        less_than_100(100);
    }

    #[test]
    fn test_secret() -> Result<(), String> {
        match get_secret_number(2) {
            Ok(_) => Ok(()),
            Err(value) => Err(value)
        }
    }

    #[test]
    fn test_add_two() {
        assert_eq!(
            add_two(5), 7,
            "The add_two function was unable to add 2 to the number 5"
        )
    }
    
    #[test]
    fn test_can_hold_fails() {
        let r1 = Rectangle {
            width: 5,
            height: 5,
        };

        let r2 = Rectangle {
            width: 8,
            height: 8,
        };

        assert!(!r1.can_hold(&r2))
    }

    #[test]
    #[ignore]
    fn test_can_hold() {
        let r1 = Rectangle {
            width: 5,
            height: 5,
        };

        let r2 = Rectangle {
            width: 3,
            height: 2,
        };

        assert!(r1.can_hold(&r2))
    }
}
