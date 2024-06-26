use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // Box<dyn Error> is basically to return any type of error
    let content = fs::read_to_string(config.file_name)?;

    println!("The content of the file is: {}", content);

    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_name: &'a str,
}

impl<'a> Config<'a> {    
    pub fn new(args: &'a[String]) -> Result<Config<'a>, String> {
        if args.len() < 3 {
            Err("Not enough arguments".to_string())
        } else {
            Ok(Config {
                query: &args[1],
                file_name: &args[2],
            })
        }
    }
    
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast and productive
pick three.";

        assert_eq!(vec!["safe, fast and productive"], search(query, content));
    }

}