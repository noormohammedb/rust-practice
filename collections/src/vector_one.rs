pub fn vector1() {
    let my_ar = [1, 2, 3];
    let mut my_vec: Vec<i32> = Vec::new();

    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    println!("my_vec: {:?}", my_vec);

    let vec_v2 = vec![1, 2, 3];
    println!("vec_v2: {:?}", vec_v2);

    let third = &vec_v2[2];
    println!("third: {}", third);

    match vec_v2.get(2) {
        Some(vec_third) => println!("vec_third: {}", vec_third),
        None => println!("vector index none : None"),
    }

    match vec_v2.get(2002) {
        Some(vec_third) => println!("vec_third: {}", vec_third),
        None => println!("vector index none : None"),
    }
}
