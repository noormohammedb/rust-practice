use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nill,
}

use List::{Cons, Nill};
pub fn run() {
    // let a = 5;
    // let b = &mut a;

    // let mut c = 10;
    // let d = &c;
    // *d = 8;

    let mut am_mutable = 22;
    let am_mutable_ref_of_mutable = &mut am_mutable;
    *am_mutable_ref_of_mutable = 22;

    // let my_tracker = LimitTracker::new()

    /*
     * multiple owners of mutable data
     */
    let value = Rc::new(RefCell::new(15));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nill)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("\na after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c)
}

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
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentae_of_max = self.value as f64 / self.max as f64;

        if percentae_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentae_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of quota!");
        } else if percentae_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenget {
        send_message: RefCell<Vec<String>>,
    }

    impl MockMessenget {
        fn new() -> MockMessenget {
            MockMessenget {
                send_message: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenget {
        fn send(&self, message: &str) {
            // println!("{}", message);
            self.send_message.borrow_mut().push(String::from(message));

            /*
             * can't borrow with mut twise it would be runtime error because of RefCell
             */
            // let mut one_borrow = self.send_message.borrow_mut();
            // let mut two_borrow = self.send_message.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_message = MockMessenget::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);

        limit_tracker.set_value(80);
        println!("{}", mock_message.send_message.borrow()[0]);

        assert_eq!(mock_message.send_message.borrow().len(), 1);
    }
}
