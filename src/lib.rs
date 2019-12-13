use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
    
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: tests for Config::new
    #[test]
    // fn new_config_no_arguments() {
    //     let args = vec![].iter();
    //     let has_error = Config::new(args).is_err();
    //     assert!(has_error);
    // }

    // #[test]
    // fn new_config_not_enough_arguments() {
    //     let args = vec![String::from("minigrep"), String::from("rust")].iter();
    //     let has_error = Config::new(args).is_err();
    //     assert!(has_error);
    // }

    // #[test]
    // fn new_config_enough_arguments() {
    //     // This is bad because a change of environment variable could cause it to fail...
    //     // Not sure how to quite fix this with my current Rust knowledge without a
    //     // "beforeEach" function that reliably sets the environment variable to a
    //     // specific value before each test, or stubbing out return values of env::var.
    //     // Or rather, is this even possible without using a third-party crate?
    //     let args = vec![String::from("minigrep"), String::from("rust"), String::from("poem.txt")];
    //     let config = Config::new(&args).unwrap();

    //     match config {
    //         Config { query, filename, case_sensitive } => {
    //             assert_eq!(query, "rust");
    //             assert_eq!(filename, "poem.txt");
    //             assert!(case_sensitive);
    //         }
    //     }
    // }

    // TODO: tests for run
    // TODO: add tests for controlling case insensitivity with command line flag
    // TODO: add test for command line flag overriding env vars

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
