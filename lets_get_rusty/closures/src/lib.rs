use std::fmt::Debug;

pub fn closure_two() {
    let x = 4;
    let y = 4;

    let equal_to_x = |z| z == x;

    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    assert!(equal_to_x(y));
}

/* closure are 3 type of
   1. Fn - immutable reference
   2. FnMut - mutable reference
   3. FnOnce - takes ownership or moves
*/

pub fn closure_three() {
    let x = vec![1, 2, 3];
    // let equal_to_x = move |z: &Vec<i32>| *z == x;

    myprint(&x);
    let equal_to_x = move |z| z == x;
    // myprint(&x);
    // println!("can't use x here: {:?}", x);

    // let mut y = vec![1, 2, 3];
    let y = vec![1, 2, 3];
    // let k = vec![1, 2, 3];
    // assert!(equal_to_x(&mut y));
    // assert!(equal_to_x(&y));
    assert!(equal_to_x(y));
}

// pub fn myprint<T: Debug>(data: Vec<T>) {
pub fn myprint<T: Debug>(data: &Vec<T>) {
    println!("data: {:?}", data);
}
