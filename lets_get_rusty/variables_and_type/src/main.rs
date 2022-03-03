mod data_type;
fn main() {
    let mut x = 44;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x);

    const COUNT: u32 = 21000;
    println!("The value of COUNT is: {}", COUNT);
    data_type::data_type();
}
