use colored::*;
use std::{borrow::Cow, env, error::Error, fs};
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let res = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in res {
        colored_print(&line);
    }
    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<SingleLineResult<'a>> {
    search(query, contents, |line| Cow::Borrowed(line))
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<SingleLineResult<'a>> {
    search(query, contents, |line| Cow::Owned(line.to_lowercase()))
}

fn search<'a, T>(query: &str, contents: &'a str, callable: T) -> Vec<SingleLineResult<'a>>
where
    T: for<'b> Fn(&'b str) -> Cow<'b, str>,
{
    let query = callable(query);
    contents
        .lines()
        .map(|line| -> (&str, Vec<_>) {
            (
                line,
                callable(line)
                    .match_indices(&*query)
                    .map(|(idx, value)| (idx, &line[idx..idx + value.len()]))
                    .collect(),
            )
        })
        .filter(|(_line, res)| !res.is_empty())
        .map(|(line, matches)| SingleLineResult { line, matches })
        .collect()
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

        assert_eq!(vec![result], search_case_sensitive(query, contents));
    }
    #[test]
    fn no_result() {
        let query = "Naharda!";
        let contents = "Not the word\nAlso not the word.";
        let cmp: Vec<SingleLineResult> = vec![];
        assert_eq!(cmp, search_case_sensitive(query, contents));
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
            search_case_sensitive(query, contents)
        );
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec![SingleLineResult {
                line: "safe, fast, productive.",
                matches: vec![(15, "duct")]
            }],
            search_case_sensitive(query, contents)
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
            vec![
                SingleLineResult {
                    line: "Rust:",
                    matches: vec![(0, "Rust")]
                },
                SingleLineResult {
                    line: "Trust me.",
                    matches: vec![(1, "rust")]
                }
            ],
            search_case_insensitive(query, contents)
        );
    }
}
