use std::{fs::File, io::ErrorKind};

pub fn error_one() {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let my_file = File::open("hello.txt");
    let my_file = File::open("Cargo.toml");

    let my_file = match my_file {
        Ok(file) => file,
        Err(file_error) => panic!("Problem opening the file: {:?}", file_error),
    };
    println!("{:?}", my_file);

    let my_no_file_name = "hello.txt";
    let my_no_file = File::open(&my_no_file_name);

    let my_no_file = match my_no_file {
        Ok(file) => file,
        Err(file_error) => match file_error.kind() {
            ErrorKind::NotFound => {
                println!("No file found at that path.");
                match File::create(&my_no_file_name) {
                    Ok(created_file) => {
                        println!("file created");
                        created_file
                    }
                    Err(cant_create_file) => {
                        panic!("can't create file : {}", cant_create_file);
                    }
                }
            }
            ErrorKind::PermissionDenied => {
                panic!("You don't have permission to open this file.");
            }
            other_error => {
                panic!("something else went wrong, other error: {:?}", other_error);
            }
        },
    };
    println!("my_no_file: {:#?}", my_no_file);

    // closures
    let f = File::open(&my_no_file_name).unwrap_or_else(|my_error| {
        if my_error.kind() == ErrorKind::NotFound {
            File::create(&my_no_file_name).unwrap_or_else(|create_error| {
                panic!("Problem creating the file: {:?}", create_error);
            })
        } else {
            panic!("Problem opening the file: {:?}", my_error);
        }
    });

    println!("f: {:?}", f);
    // panic if can't open file
    let f = File::open(&my_no_file_name).unwrap();
    // pass panic message to caller
    let f = File::open(&my_no_file_name).expect("Failed to open the file");
}
