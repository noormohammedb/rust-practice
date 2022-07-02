mod destructuring;
mod ignoring_values;
mod match_guard;
mod pattern_syntax;
fn main() {
    println!("Match Experssion:");

    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    // let language = Language::English;
    let language = Language::Japanese;

    match language {
        Language::English => println!("Hello World!"),
        Language::Japanese => println!("Hola Mundo!"),
        Language::Russian => println!("Привет, мир!"),
        lang => println!("Unsuported language! {:?}", lang),
    }

    println!("\nConditional if let Expression:");

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    println!("\nWhile let Conditional Loops:");

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    println!("\nfor Loops:");

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        /*
        for value in v.iter()
        println!("{} is at index ", value);
        */
        println!("{} is at index {}", value, index);
    }

    /* let statements
     */

    let x = 5;

    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);
    let (a, b, _) = (7, 8, 9); // ignoring last variable

    println!("\nfunction parameters:");

    let point = (3, 5);
    print_coordinates(&point);

    // Irrefutable
    let x = 5;

    // Refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // Can only accept irreutable patterns:
    /* - function parameters
     * - let statements
     * - for loops
     */

    pattern_syntax::run();
    destructuring::run();
    ignoring_values::run();
    match_guard::run();
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location : ({}, {})", x, y);
}
