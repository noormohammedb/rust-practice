struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Msg_color {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub fn run() {
    let p = Point { x: 0, y: 7 };

    let Point { x: foo, y: bar } = p;

    assert_eq!(0, foo);
    assert_eq!(7, bar);

    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x);
        }
        Point { x: 0, y } => {
            println!("On the y axis at {}", y);
        }
        Point { x, y } => {
            println!("On neither axis: ({}, {})", x, y);
        }
    }

    // enum destructuring

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Move to x: {} y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color: red {}, green {} and blue {}", r, g, b);
        }
    }

    let message_with_color = Msg_color::ChangeColor(Color::Hsv(0, 160, 255));

    match message_with_color {
        Msg_color::Quit => {
            println!("Quit");
        }
        Msg_color::Move { x, y } => {
            println!("move")
        }
        Msg_color::Write(text) => {
            println!("Text message: {}", text);
        }
        Msg_color::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color enum: red {}, green {} and blue {}", r, g, b);
        }
        Msg_color::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change color enum: hue: {}, saturation: {}, value: {}",
                h, s, v
            );
        }
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 4, y: -10 });
}
