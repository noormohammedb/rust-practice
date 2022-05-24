pub fn my_gen() {
    let number_list = vec![23, 43, 13, 90, 23, 10, 98];
    let largerst = find_larg(number_list);
    println!("The largest number is {}", largerst);

    let number_list = vec![42, 84, 271, 81, 3, 42, 12];
    let largerst = find_larg(number_list);
    println!("The largest number is {}", largerst);

    let char_list = vec!['f', 'o', 'o', 'b', 'a', 'r'];
    let largest = find_larg(char_list);
    println!("The largest number is {}", largerst);
}

pub fn find_larg<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut large = list[0];

    for list_iter in list {
        if list_iter > large {
            large = list_iter;
        }
    }

    large
}
