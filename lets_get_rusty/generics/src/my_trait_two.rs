use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // fn cmp_display(&self) {
    fn cmp_display(data: &Self) {
        if data.x > data.y {
            println!("x is greater that y, {}>{}", data.x, data.y);
        } else {
            println!("bar");
            println!("x is smaller that y, {}<{}", data.x, data.y);
        }
    }
}

// impl<T> Display for Pair<T> {
//     // quick fix
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }

// }
// impl<T> PartialOrd for Pair<T> {}
// impl<T> PartialOrd for Pair<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         None(());
//     }
// }

pub fn trait_two() {
    let my_pair = Pair::new(4, 1);

    // my_pair.cm
}
