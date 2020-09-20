use colored::*;
use std::{error::Error, fs};
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Please supply pattern and filename!");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &contents) {
        //println!("{}", line.line);
        colored_print(&line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<SingleLineResult<'a>> {
    let mut results = Vec::new();

    for line in contents.lines() {
        let matches: Vec<_> = line.match_indices(query).collect();
        if !matches.is_empty() {
            results.push(SingleLineResult { line, matches })
        }
    }

    results
}

fn colored_print(line: &SingleLineResult) {
    let mut curr_idx: usize = 0;
    for (idx, value) in line.matches.iter() {
        print!(
            "{}{}",
            &line.line[curr_idx..*idx],
            &line.line[*idx..*idx + value.len()].red().bold()
        );
        curr_idx = *idx + value.len();
    }
    println!("{}", &line.line[curr_idx..line.line.len()]);
}
#[derive(PartialEq, Debug)]
pub struct SingleLineResult<'a> {
    line: &'a str,
    matches: Vec<(usize, &'a str)>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let result = SingleLineResult {
            line: "safe, fast, productive.",
            matches: vec![(15, "duct")],
        };

        assert_eq!(vec![result], search(query, contents));
    }
    #[test]
    fn no_result() {
        let query = "Naharda!";
        let contents = "Not the word\nAlso not the word.";
        let cmp: Vec<SingleLineResult> = vec![];
        assert_eq!(cmp, search(query, contents));
    }
    #[test]
    fn multiple_results() {
        let query = "Naharda!";
        let contents = "I am talking to you, Naharda!
NahardaNaharda!
Naharda!Naharda!
Naharda! Naharda Naharda!";
        assert_eq!(
            vec![
                SingleLineResult {
                    line: "I am talking to you, Naharda!",
                    matches: vec![(21, "Naharda!")]
                },
                SingleLineResult {
                    line: "NahardaNaharda!",
                    matches: vec![(7, "Naharda!")]
                },
                SingleLineResult {
                    line: "Naharda!Naharda!",
                    matches: vec![(0, "Naharda!"), (8, "Naharda!")]
                },
                SingleLineResult {
                    line: "Naharda! Naharda Naharda!",
                    matches: vec![(0, "Naharda!"), (17, "Naharda!")]
                }
            ],
            search(query, contents)
        );
    }
}
