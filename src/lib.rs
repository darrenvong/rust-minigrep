use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // Calling .clone() is needed so that Config can own the values of `query`
        // and `filename` without violating the borrowing rule.
        //
        // Not the most efficient solution, but ok for now for simplicity.
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    println!("With text:\n{}", contents);

    Ok(())
}
