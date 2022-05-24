#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<L, M>(self, other: Point<L, M>) -> Point<T, M> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn my_struct() {
    let p1 = Point { x: 2, y: 9 };
    let p2 = Point { x: "foo", y: 9.0 };
    let p3 = Point { x: 1, y: 7.2 };

    let mix_p = p2.mixup(p3);

    println!("mix_p: {:?}", mix_p);
}

enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
