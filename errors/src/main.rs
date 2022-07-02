mod propagation;

use enum_error::error_one;
use propagation::error_two;
// use std::io;
use std::{error::Error, net::IpAddr};
// use std::{fs::File, io::Read};

mod enum_error;
// fn main() -> Result<String, io::Error> {
fn main() -> Result<(), Box<dyn Error>> {
    a();
    error_one();
    println!("read_username: {:?}", error_two::read_username_from_file());
    println!(
        "read_username_two: {:?}",
        error_two::read_username_from_file_two()
    );

    println!(
        "read_username_three: {:?}",
        error_two::read_username_from_file_three()
    );

    // let mut file_data = String::new();
    // File::open("foo.txt")?.read_to_string(&mut file_data)?;
    // Ok(file_data)

    let localhost: IpAddr = "127.0.0.1".parse().unwrap();
    println!("localhost: {}", localhost);
    Ok(())
}
fn a() {
    b();
}

fn b() {
    c(21);
}
fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
    // println!("hello, num: {}", num);
}
