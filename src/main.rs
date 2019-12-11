use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
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

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}
