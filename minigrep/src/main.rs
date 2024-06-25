use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect(); //In collections we need to specify the expected type

    let config = Config::new(&args);

    let content = fs::read_to_string(config.file_name).expect(&format!("File not found: {}", config.file_name));

    println!("The content of the file is: {}", content);
}

struct Config<'a> {
    _query: &'a str,
    file_name: &'a str,
}

impl<'a> Config<'a> {    
    fn new(args: &'a[String]) -> Config<'a> {
        Config {
            _query: &args[1],
            file_name: &args[2],
        }
    }
    
}