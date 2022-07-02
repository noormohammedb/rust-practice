pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations
}

#[test]
fn iterator_demonstration() {
    let mut v1 = vec![1, 2, 3];
    // let v1 = vec![0, 1, 2]; // wrong

    // let mut v1_iter = v1.iter(); // reference
    // let mut v1_iter = v1.into_iter();    // ownership
    let mut v1_iter = v1.iter_mut(); // mutable reference

    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i64 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn adaper_method() {
    let v1: Vec<i64> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|arg| arg + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for data in v1_iter {
        println!("{}", data);
    }
}
