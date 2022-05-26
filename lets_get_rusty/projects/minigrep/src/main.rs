use minigrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let my_config = Config::new(&args).unwrap_or_else(|err| {
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
