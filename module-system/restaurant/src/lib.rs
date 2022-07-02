mod front_of_house;

// export
pub use crate::front_of_house::hosting;

// importing 1
/*
use rand::CryptoRng;
use rand::ErrorKind::Transient;
use rand::Rng;
*/
// nested paths
use rand::{CryptoRng, ErrorKind::Transient, Rng};

//

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 1001);

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    self::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

fn server_order() {
    println!("server_order");
}

pub fn call_thrise() {
    front_of_house::hosting::add_to_waitlist();

    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    use self::front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();
}

pub mod back_of_house;

pub fn eat_at_restaurant_second() {
    let mut meal = back_of_house::Breakfast::summer("foo");
    println!("eat_at_restaurant_second: {:#?}", meal);
    meal.toast = String::from("bar");

    /* can't create object from Breakfast struct
     * because one of the element is private
     */
    // let meal2 = back_of_house::Breakfast {
    //     toast: String::from("foo bar koo"),
    //     seasonal_fruit: String::from("lorem"), // private item
    // };

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// rename import 2
/*
use std::fmt::Result;
use std::io::Result as IoResult;
*/
use std::{fmt::Result, io::Result as IoResult};

// import 3
/*
use std::io;
use std::io::Write;
*/

use std::io::{self, Write};
/*
for import all from a module (all public items in the module)
use std::io::*;
*/

fn function1() -> Result {
    return Ok(());
}
fn function2() -> IoResult<()> {
    return Ok(());
}
