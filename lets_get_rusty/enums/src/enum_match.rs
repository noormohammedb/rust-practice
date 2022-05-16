pub fn enum_match() {
    println!("===============enum_match===============");

    let five = Some(5);
    // let si = pluse_one(&five);
    // let non = pluse_one(&None);

    let si = pluse_one(five);
    let non = pluse_one(None);

    println!("si: {:?}", si);
    println!("non: {:?}", non);

    // if let

    let some_value = Some(5);

    match some_value {
        Some(3) => println!("Three from match expression"),
        _ => println!("not found"),
    }

    if let Some(3) = some_value {
        println!("Three from if let")
    }
}

fn pluse_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(6) => {
            println!("Gotcha 6");
            Some(7)
        }
        Some(i) => {
            println!("data from match some i: {:#?}", i);
            Some(i + 1)
        }
        _ => None,
    }
}
