use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String
}

fn parse_configs(args: &[String]) -> Config {
    // Calling .clone() is needed so that Config can own the values of `query`
    // and `filename` without violating the borrowing rule.
    //
    // Not the most efficient solution, but ok for now for simplicity.
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_configs(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
}
