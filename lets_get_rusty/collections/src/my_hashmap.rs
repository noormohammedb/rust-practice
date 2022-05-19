use std::collections::HashMap;

pub fn hashmap_one() {
    println!("\nHashMap");

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // println!("blue {}", blue); // can't access

    let team_name = String::from("Blue");
    let my_team_score = scores.get(&team_name);

    println!("my_team_score: {:?}", my_team_score);

    println!("values in hashmap");
    for (key, my_val) in scores {
        print!("{}->{} ,", key, my_val);
    }

    let mut my_scores = HashMap::new();

    my_scores.insert(String::from("Blue"), 90);
    my_scores.insert(String::from("Blue"), 23); // replace 90

    my_scores.entry(String::from("Yellow")).or_insert(90); // insert if not exist
    my_scores.entry(String::from("Yellow")).or_insert(32); // can't replace 90 because of or_insert the key exist
    println!("\nscores: {:?}", my_scores);

    let my_text = "Hello world wonderful world";
    println!("my_text: {}", my_text);

    let mut my_map = HashMap::new();

    for my_word in my_text.split_whitespace() {
        let count = my_map.entry(my_word).or_insert(0);
        *count += 1;
    }
    println!("my_map: {:?}", my_map);
}
