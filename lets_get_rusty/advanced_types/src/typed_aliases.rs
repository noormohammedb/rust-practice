pub fn run() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x+y={}", x + y);

    // =======

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let _g: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type(_f: Thunk) {
        // --snip--
    }

    /*
    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    fn returns_long_type() -> Thunk {
        // --snip--
    }
    */
}
