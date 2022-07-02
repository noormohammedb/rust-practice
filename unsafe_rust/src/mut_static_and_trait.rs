static HELLO_WORLD: &str = "Hello, World!";

static mut COUNTER: u32 = 0;

pub fn run() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(2);

    unsafe { println!("COUNTER is: {}", COUNTER) }
}

fn add_to_count(inc: u32) {
    unsafe { COUNTER += inc }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
