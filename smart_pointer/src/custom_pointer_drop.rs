struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

pub fn run() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("drop me"),
    };
    println!("CustomSmartPointers created.");
    drop(e);
    println!("dropped `drop me`");
}
