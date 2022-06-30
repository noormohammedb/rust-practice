extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn run() {
    println!("external call");
    unsafe {
        println!("Absolute value of -3 according ot C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_rust_from_c() {
    println!("Just called a Rust function from C!");
}
