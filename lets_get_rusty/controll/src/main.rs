fn main() {
    let my_number = -2;

    // if-else
    if my_number < 10 {
        println!("Number is less than 5");
    } else if my_number < 20 {
        println!("Number is less than 20");
    } else {
        println!("number is greater than all condition");
    }

    //if-else inside let
    let condition = true;
    let number_from_condition = if condition { 4 } else { 18 };
    println!(
        "\ncondition: {} \nnumber: {}",
        condition, number_from_condition
    );

    // loops
    let mut counter = 0;
    let result = loop {
        print!("{}.inside Loop\t", counter);
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };
    println!("\nresult from loop counter : {}\n", result);

    // while loop
    let mut my_number = 4;
    while my_number > 0 {
        print!("{}.while loop\t", my_number);
        my_number -= 1;
    }

    // for loop / for in loop

    let my_array = [1, 2, 3, 4, 5];
    for my_ele in my_array.iter() {
        print!("\n{}", my_ele);
    }
    println!("\nFor In Loop");
    /*
     * end range are exclusive
     */
    for my_number in 1..6 {
        print!("\n{}", my_number);
    }
}
