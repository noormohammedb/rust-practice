pub fn vector2() {
    let mut my_vect = vec![1, 2, 3, 4, 5];

    for i in &my_vect {
        print!("{} ", i);
    }
    println!();

    for i in &mut my_vect {
        *i = *i * 10;
    }

    for i in &my_vect {
        print!("{} ", i);
    }
    println!();
}
