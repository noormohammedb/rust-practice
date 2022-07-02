trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait IteratorGen<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

impl IteratorGen<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}
impl IteratorGen<u128> for Counter {
    fn next(&mut self) -> Option<u128> {
        Some(0)
    }
}
pub fn run() {
    println!("");
}
