#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
pub fn run() {
    foo(3, 5);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    let my_nums = (2, 4, 8, 16, 32);

    match my_nums {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let _x = 5;
    let y = 9;

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("got a string",);
    }

    // println!("{:?}", s);

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => {
            println!("x is {}", x);
        }
    }

    let my_numbers = (2, 4, 6, 7);

    match my_numbers {
        (first, .., last) => println!("first : {}, last: {}", first, last),
    }

}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {} ", y);
}
