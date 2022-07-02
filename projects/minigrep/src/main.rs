use minigrep::Config;
use std::{env, process};
fn main() {
    let mut my_config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumnts: {}", err);
        process::exit(-1);
    });
    // println!("arg: {:?}", args);
    // println!("query : {}", my_config.query);
    // println!("filename : {}", my_config.filename);

    if let Err(e) = minigrep::run(my_config) {
        eprintln!("Application Error : {}", e);
        process::exit(2);
    }
}
