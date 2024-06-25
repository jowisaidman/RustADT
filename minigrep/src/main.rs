use std::{env, error::Error, fs, process};

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

fn run(config: Config) -> Result<(), Box<dyn Error>> { // Box<dyn Error> is basically to return any type of error
    let content = fs::read_to_string(config.file_name)?;

    println!("The content of the file is: {}", content);

    Ok(())
}

struct Config<'a> {
    _query: &'a str,
    file_name: &'a str,
}

impl<'a> Config<'a> {    
    fn new(args: &'a[String]) -> Result<Config<'a>, String> {
        if args.len() < 3 {
            Err("Not enough arguments".to_string())
        } else {
            Ok(Config {
                _query: &args[1],
                file_name: &args[2],
            })
        }
    }
    
}