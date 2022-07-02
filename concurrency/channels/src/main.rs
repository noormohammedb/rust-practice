use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("1.hi"),
            String::from("1.from"),
            String::from("1.the"),
            String::from("1.thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2.more"),
            String::from("2.message"),
            String::from("2.for"),
            String::from("2.you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // println!("Got: {}", received);

    for data in rx {
        println!("Got: {}", data);
    }
}
