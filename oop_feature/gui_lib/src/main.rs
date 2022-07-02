use gui_lib::{Button, Draw, Screen};
use std::string::String;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

fn main() {
    let my_screen = Screen {
        components: vec![
            Box::new(SelectBox {
                height: 100,
                width: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                height: 100,
                width: 100,
                label: String::from("ok"),
            }),
        ],
    };

    my_screen.run();
}
