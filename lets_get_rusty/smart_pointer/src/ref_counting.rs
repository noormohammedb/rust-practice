use smart_pointer::ListRc::{Cons, Nil};
use std::rc::Rc;

pub fn run() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    // let b = Cons(3, a.clone());
    println!("count after creating b {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c {}", Rc::strong_count(&a));
    }
    println!("count after block c {}", Rc::strong_count(&a));

    let d = Cons(4, Rc::clone(&a));
    println!("count after creating d {}", Rc::strong_count(&a));
}
