#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn add_two(arg: i32) -> i32 {
    arg + 2
}

pub fn greeting(arg_as_name: &str) -> String {
    format!("Hello {}!", arg_as_name)
    // format!("Hello {}!", String::from(""))
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
    // #[test]
    // fn failing_test() {
    //     panic!("Make this test fail");
    // }

    use crate::*;

    use super::Rectangle;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 4,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(3, add_two(1));
        assert_ne!(9, add_two(5));
    }

    #[test]
    fn greeting_contains_name() {
        let data = String::from("foo");
        let result = greeting(data.as_str());
        assert!(
            result.contains(data.as_str()),
            "Greeting did not contain name: {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_two() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        // if 2 + 3 == 3 {
        if 2 + 3 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus three does not equal three"))
        }
    }
}
