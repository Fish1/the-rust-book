pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: you are over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: you are over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: std::cell::RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: std::cell::RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message));
            
            self.sent_messages.borrow_mut().push(String::from(message));
            // we can do this second line because the first borrow goes out
            // of scope when the second borrow happens
            self.sent_messages.borrow_mut().push(String::from(message));
            
            // the following code will cause a runtime panic
            // ass the RefCell will have detected that two
            // mutable borrows exists
            /*
            let mut b1 = self.sent_messages.borrow_mut();
            let mut b2 = self.sent_messages.borrow_mut();
            b1.push(String::from(message));
            b2.push(String::from(message));
            */
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(
            &mock_messenger, 100
        );

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);
    }
}


