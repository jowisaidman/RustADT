use std::{error::Error, fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // Box<dyn Error> is basically to return any type of error
    let content = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitve {
        search(config.query, &content)
    } else {
        search_case_insensitive(config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_name: &'a str,
    pub case_sensitve: bool,
}

impl<'a> Config<'a> {    
    pub fn new(args: &'a[String]) -> Result<Config<'a>, String> {
        let case_sensitve = env::var("CASE_INSENSITIVE").is_err();
        if args.len() < 3 {
            Err("Not enough arguments".to_string())
        } else {
            Ok(Config {
                query: &args[1],
                file_name: &args[2],
                case_sensitve
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

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast and productive
pick three.
Duct quack";

        assert_eq!(vec!["safe, fast and productive"], search(query, content));
    }

    #[test]
    fn case_insensitve() {
        let query = "RusT";
        let content = "\
rust is a great
language.
All people should use RuSt";

        assert_eq!(vec!["rust is a great", "All people should use RuSt"], search_case_insensitive(query, content))
    }

}