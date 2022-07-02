use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

pub fn string_one() {
    // string are UTF-8 encoded byte
    let st1 = String::new();
    let st2 = "initial";
    let st3 = st2.to_string();
    let st4 = String::from("first content");

    let hello = "Hello";
    let hello = "नमस्ते";
    let hello = "Здравствуйте";

    // string can grow and shrink
    let mut my_string = String::from("foo");
    my_string.push_str(" bar");
    my_string.push('!');

    println!("my_string: {}", my_string);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = format!("{}{}", s1, s2); // can't take ownership of s1 and s2 because of macro

    println!("s3: {}", s3);
    let s4: String = s1 + &s2; // takes ownership of s1

    println!("s4: {}", s4);

    let hello = String::from("नमस्ते");
    // let c: char = hello.get[0];  // can't work
    println!("hello: {}", hello);

    // bytes
    print!("bytes of {} : ", hello);
    for byte_iter in hello.bytes() {
        print!("{} ", byte_iter);
    }

    // characters (scalar values)
    print!("\ncharacters of {} : ", hello);
    for char_iter in hello.chars() {
        print!("{} ", char_iter);
    }

    // Grapheme clusters
    print!("\ngraphime clusters of {} : ", hello);
    for cluster_iter in hello.graphemes(true) {
        print!("{} ", cluster_iter);
    }
}
