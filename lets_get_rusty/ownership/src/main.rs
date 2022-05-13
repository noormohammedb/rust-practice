fn main() {
    // stack and heap
    fn a() {
        let x = "hello"; // x (fixed size) -> stack
        let y = 22; // y (fixed size) -> stack
        b();
    }

    fn b() {
        let x = String::from(",world!");
        /* x (variable size) -> heap
         * x stores a pointer to a String object on the heap
         */
    }

    /* ownership rules
     * 1. Each value in Rust has a variable that’s called its owner.
     * 2. There can only be one owner at a time.
     * 3. When the owner goes out of scope, the value will be dropped.
     */
    {
        // s is not valid here, it's not yet declared
        let s = "Hello"; // s is valid from this point forward
                         // do stuff with s
    } // this scope is now over, and s is no longer valid

    /* ownership move
     * simple types stores on stack copys value
     * int, bool, char are copied by value
     */
    {
        let x = 3;
        let y = x; // copy
        println!("x: {}, y: {}", x, y);

        println!("Move:");
        let s1 = String::from("Hello");
        let s2 = s1;
        // println!("s1: {}, s2: {}", s1, s2);  // value borrowed here after move
        println!("s2: {}", s2);
        println!("Clone:");

        // clone data
        let s1 = String::from("Hello");
        let s2 = s1.clone();
        println!("s1: {}, s2: {}", s1, s2);
    }

    /* Ownership in functions
     */

    let s = String::from("my string");
    takes_ownership(s);
    // println!("s: {}", s); // s is moves to str_arg in takes_ownership function

    let x = 9;
    makes_copy(x);
    println!("x: {}", x); // x is copied to x_arg in makes_copy function
                          // integers are copied by value in rust

    // example 2
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s1: {}, s2: {}, s3: {}", s1, s2, s3); // cant use s2 here because it was moved to giv_bak in takes_and_gives_back function
    println!("s1: {},  s3: {}", s1, s3);

    // References and Borrowing
    let s1 = String::from("lorem");
    let leng = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, leng);

    /* Mutable reference
     * a reference can borrow as immutable multiple times
     * a reference can borrow once as mutable
     * if a reference borrowed as mutable then it can't be borrowed as immutable
     * if a reference borrowed as immutable then it can't be borrowed as mutable
     * (that means if a reference is borrowed as mutable once. then it can't be borrowed any again)
     * (if immutable reference is borrowed any times, can't borrow as mutable)
     *
     * A Data Have a one mutable reference or any no.of immutable references in a scope at a time
     * and those reference must be valid (not be dangle reference)
     */
    let mut s1 = String::from("foo");
    change(&mut s1);
    println!("s1: {}", s1);

    // example 2 of mutable reference
    let mut s1 = String::from("hello");

    let r1 = &s1;
    // let r2 = &mut s1; // a mutable reference can't borrow more than once
    // let r3 = &mut s1; // a mutable reference can't borrow more than once
    // let r4 = &mut s1; // can't borrow as immutable because it was already borrowed as mutable

    my_print_ref(&s1);
    println!("s1: {}", s1);
    println!("r1: {}", r1);

    let r5 = &mut s1; // possible because immutable reference r1 scope ends
    println!("r5: {}", r5);

    // dangling reference
    // let reference_to_nothing = dangle();
    // println!("reference_to_nothing: {}", reference_to_nothing);

    /* Slices
     * refeferences to a contiguous sequence of elements in a collection,
     * insted of ref.ing entire collection
     *
     * slices don't take ownership of the data
     * returns first word of the string
     */
    let mut my_string = String::from("hello world");
    let first_word_size = first_word(&my_string);
    println!("my_string: {}", &my_string);
    println!("first_word_size: {}", &first_word_size);
    my_string.clear();
    println!("my_string: {}", &my_string);
    println!("first_word_size: {}", &first_word_size);

    let mut s = String::from("hello world");
    let hello = &s[..5]; // 5 is excluded
    let world = &s[..]; // string slice of entier string

    let word = first_word2(&s);
    println!("s: {}", s);
    println!("word: {}", word);
    // s.clear(); // can't mutate because there is a immutable reference
    println!("s: {}", s);
    println!("word: {}", word);

    let s2 = "hello world";
    let word = first_word3(s2);

    // slices on different collections
    let my_coll = [1, 2, 3, 4, 5];
    let my_sliec = &my_coll[..2];
}

fn takes_ownership(str_arg: String) {
    println!("str_arg: {}", str_arg);
}

fn makes_copy(x_arg: i32) {
    println!("x_arg: {}", x_arg);
}

fn gives_ownership() -> String {
    let some_string = String::from("foo bar");
    some_string
}

fn takes_and_gives_back(giv_bak: String) -> String {
    giv_bak
}

fn calculate_length(s: &String) -> usize {
    // s.push_str("ipsum"); // can't mutate its just a borrowed reference
    let str_length = s.len();
    str_length
}

fn change(some_string: &mut String) {
    some_string.push_str("Foo");
}

fn my_print(my_str: String) {
    println!("my_str: {}", my_str);
}

fn my_print_ref(my_str: &String) {
    println!("ref my_str: {}", my_str);
}

fn my_print_mut_ref(my_str: &mut String) {
    println!("mut ref my_str: {}", my_str);
}

// fn dangle() -> &String {
//     let my_dangle_string = String::from("dangle");
//     &my_dangle_string
// }

fn first_word(s: &String) -> usize {
    let my_bytes = s.as_bytes();

    for (i, &item) in my_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let my_bytes = s.as_bytes();

    for (i, &item) in my_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word3(s: &str) -> &str {
    let my_bytes = s.as_bytes();

    for (i, &item) in my_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
