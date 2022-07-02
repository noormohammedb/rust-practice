use std::{thread, time::Duration};
fn main() {
    let handle_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle_thread.join().unwrap();

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_micros(1000));
    }

    println!("\n");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
}
