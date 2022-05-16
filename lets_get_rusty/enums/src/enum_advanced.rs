// options
enum MyOption<T> {
    Some(T),
    None,
}

// Match Expression
#[derive(Debug)]
enum UsState {
    Alaska,
    California,
    Colorado,
    Florida,
    Georgia,
    Hawaii,
    Indiana,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            return 1;
        }
        Coin::Nickel => return 5,
        Coin::Dime => return 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}", state);
            return 25;
        }
        _ => return 0,
    };
}

pub fn enum_advanced() {
    println!("===============enum_advanced===============");

    let some_number = Some(54);
    let some_string = Some(String::from("foo bar koo"));

    let absent_number: Option<i32> = None;

    let x: i8 = 3;
    // let y: Option<i8> = Some(9);
    let y: Option<i8> = None;

    let sum = x + y.unwrap_or(0);
    println!("sum: {}", sum);

    let coin_data = value_in_cents(Coin::Penny);
    println!("coin_data: {}", coin_data);

    let state_data = value_in_cents(Coin::Quarter(UsState::Indiana));
    println!("state_data: {}", state_data);
}
