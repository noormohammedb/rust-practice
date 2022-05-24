pub fn lifetime_one() {
    // {
    // let r; //     ------------------+-- 'a
    //        //                             |
    // {
    //     //                                |
    //     let x = 5; // ----+-- 'b     |
    //     r = &x; //             |          |
    // } //                      -+          |
    //   //                                  |
    // println!("r: {}", r); //              |
    // } //                              --------+

    let string1 = String::from("foo"); // 'a
    let string2 = String::from("bar"); // 'b

    let result1 = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result1);

    {
        let string3 = String::from("long string is long");
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("The longest string is {}", result2);
    }

    /*
    let result3: &str;
    {
        let string4 = String::from("longest string");
        result3 = longest(string2.as_str(), string4.as_str());
        /* smallest lifetime is string4 is outof scope
         * lifetime of string4 is out of scope, there for result3 would be dangling reference
         */
    }
    println!("restult3: {}", result3);
     */

    // lifetime for structure field
    let novel = String::from("Call me foo. Some years ao...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let imp_sen = ImportantExcerp {
        part: first_sentence,
    };

    println!("imp_sen: {}", imp_sen.part);

    let s: &'static str = "I have a static lifetime.";
    println!("static life time, s: {}", s);
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerp<'a> {
    part: &'a str,
}
/* lifetime elision rules
 *  1. Each parameter that is a reference gets its own lifetime parameter.
 *
 *  2. if there is exactly one input lifetime parameter, that lifetime is
 *     assigned to all output lifetime parameters.
 *
 *  3. if there are multiple input lifetime parameters, but one of them is
 *     is self reference or mutable self reference, the lifetime of self is
 *     assigned to all output lifetime parameters.
 *
 *
 * below function, does not have to specify lifetime parameter
 * because it is only one input lifetime parameter (satisfied by rule 2)
 */

fn first_word<'a>(s: &'a str) -> &'a str {
    let my_bytes = s.as_bytes();

    for (i, &item) in my_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

impl<'a> ImportantExcerp<'a> {
    /* does not have to specify lifetime parameter
     * because it have a self reference (satisfied by rule 3)
     */
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
