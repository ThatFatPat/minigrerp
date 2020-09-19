use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!(
        "Searching for {}\nIn file {}",
        config.query, config.filename
    );

    let contents = fs::read_to_string(&config.filename)
        .expect(&format!("Could not read file {}", config.filename)[..]);

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}
