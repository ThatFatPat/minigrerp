use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for {}\nIn file {}",
        config.query, config.filename
    );

    run(config);
}
fn run(config: Config) {
    let contents = fs::read_to_string(&config.filename)
        .expect(&format!("Could not read file {}", config.filename)[..]);

    println!("With text:\n{}", contents);
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Please supply pattern and filename!");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
