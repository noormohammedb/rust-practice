use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // let m = Mutex::new(4);
    // println!("m = {:?}", m);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 3;
    // }

    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let my_counter = Rc::clone(&counter); // it's not thread safe
        let my_arc_counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = my_arc_counter_clone.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter: {}", counter.lock().unwrap());
}
