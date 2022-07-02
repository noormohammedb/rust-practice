use std::slice;

pub fn run() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let _r3 = address as *const i32;

    /*
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    */

    unsafe fn dangerous(r1: *const i32, r2: *mut i32) {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    unsafe { dangerous(r1, r2) }

    // safe abstraction

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut w = vec![1, 2, 3, 4, 5, 6];
    let (c, d) = split_at_mut(&mut w, 3);

    assert_eq!(c, &mut [1, 2, 3]);
    assert_eq!(d, &mut [4, 5, 6]);
}

/*
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    unsafe { (&mut slice[..mid], &mut slice[mid..]) }
}
*/

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
