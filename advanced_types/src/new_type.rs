use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct _Age(u32);
struct _ID(u32);

pub fn run() {
    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);

    println!("w = {}", w);
}
