use std::{env};
use simple_error::bail;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type DayResultsStr = (Option<String>, Option<String>);
pub type DayResults = (Option<i64>, Option<i64>);

#[derive(Debug)]
pub struct Opts {
    pub input_suffix: String,
    pub verbose: bool,
    pub gold: bool,
}

pub fn parse_args() -> Result<Opts> {
    let mut input_suffix: String = "ex".into();
    let mut verbose = false;
    let mut gold = false;

    let args = env::args().skip(1);
    for a in args {
        match a.as_str() {
            "-v" | "--verbose" => verbose = true,
            "-g" | "--gold" => gold = true,
            _ if a.chars().nth(0) != Some('-') => input_suffix = a,
            _ => bail!("Invalid argument \"{}\"", a)
        }
    }

    return Ok(Opts{input_suffix, verbose, gold})
}

pub fn print_str_results(results: Result<DayResultsStr>) {
    match results {
        Err(e) => println!("\x1b[91mError:\x1b[0m {}", e),
        Ok((s, g)) => {
            if let Some(s) = s { println!("🥈: \x1b[92m{}\x1b[0m", s); }
            if let Some(g) = g { println!("🥇: \x1b[92m{}\x1b[0m", g); }
        }
    }
}

pub fn print_results(results: Result<DayResults>) {
    fn fmt_num(v: i64) -> Option<String> { Some(format!("{}", v)) }
    print_str_results(
        results.and_then(|(silver, gold)| 
            Ok((
                silver.and_then(fmt_num), 
                gold.and_then(fmt_num) 
            ))
        )
    )
}