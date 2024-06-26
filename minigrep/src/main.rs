use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect(); //In collections we need to specify the expected type

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing argumnents: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error while reading the file: {}", e);
        process::exit(1);
    };
}
