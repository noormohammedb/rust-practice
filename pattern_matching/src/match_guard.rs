enum Message {
    Hello { id: i32 },
}

pub fn run() {
    let num = Some(5);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("Yes"),
        _ => println!("No"),
    }

    let msg = Message::Hello { id: 4 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found and id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
