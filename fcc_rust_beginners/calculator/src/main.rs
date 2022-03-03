use std::env::{args, Args};

fn main() {
    let mut args_var: Args = args();

    let first_arg = args_var.nth(1).unwrap();
    let second_arg = args_var.nth(0).unwrap();
    let third_arg = args_var.nth(0).unwrap();

    let first_num = first_arg.parse::<f32>().unwrap();
    let second_num = third_arg.parse::<f32>().unwrap();

    let oper = second_arg.chars().next().unwrap();

    let result = operate(oper, first_num, second_num);

    println!("{} {} {} = {}", first_num, oper, third_arg, result);

    // println!("{:?}", args_var);
    // println!("{} {} {}", first_arg, second_arg, third_arg);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    // if operator == '+' {
    //     // println!("Addition");
    //     // println!("{}", first_number + second_number);
    //     return first_number + second_number;
    // } else if operator == '-' {
    //     // println!("Subtraction");
    //     // println!("{}", first_number - second_number);
    //     first_number - second_number // it returns the value of the expression /*-  without ; -*/
    // } else if operator == '/' {
    //     // println!("Division");
    //     // println!("{}", first_number / second_number);
    //     first_number / second_number
    // } else if operator == '*' || operator == 'x' || operator == 'X' {
    //     // println!("Multiplication");
    //     // println!("{-}", first_number * second_number);

    //     first_number * second_number
    // } else {
    //     println!("Invalid operator");
    //     println!("Input Error!");

    //     0.0
    // }

    // rusts pattern matching expression

    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => {
            print!("Invalid Operator");
            panic!("Invalid Operator Used");
        }
    }
}
