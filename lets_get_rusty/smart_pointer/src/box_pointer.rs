use std::ops::{Deref, DerefMut};

use smart_pointer::List::{Cons, Nil};

pub fn run() {
    let foo = Box::new(5);
    println!("foo: {}", foo);

    let my_list = Cons(3, Box::new(Cons(4, Box::new(Nil))));

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 9;
    let y = MyBox::new(x);

    assert_eq!(9, x);
    assert_eq!(9, *y);
    // assert_eq!(9, *(y.deref()));

    let m = MyBox::new(String::from("Foo Bar Koo..."));

    hello(&m);
    // &MyBox<String> ->&String // dereference
    // hello(&(*m)[..]);
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }

    // fn deref(&self) -> &T {
    //     &self.0
    // }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_mut(name: &mut str) {
    println!("Hello, {}!", name);
}
