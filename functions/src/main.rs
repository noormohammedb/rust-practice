fn main() {
    my_function();
    my_function_1(3, 4);
    let val_1 = fun_with_return_1(4, 2);
    println!("{}", val_1);
    let val_2 = fun_with_return_2(4, 2);
    println!("{}", val_2);
    let val_3 = fun_with_return_3(3, 1);
    println!("{}", val_3);
}

fn my_function() {
    println!("Function. This is a test");
}

fn my_function_1(x: i32, y: i32) {
    println!("Another Function. with 2 arg {} {}", x, y);
}

fn fun_with_return_1(x: i32, y: i32) -> i32 {
    let sum: i32 = x + y;
    return sum;
}

fn fun_with_return_2(x: i32, y: i32) -> i32 {
    let sum: i32 = x + y;
    sum
}

fn fun_with_return_3(x: i32, y: i32) -> i32 {
    x + y
}
