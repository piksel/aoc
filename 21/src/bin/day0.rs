use std::io::{self, prelude::*};
use std::fs::File;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const DAY_NUM: i32 = 0;

fn main() {
    println!("\x1b[32mAoC \x1b[36m21\x1b[0m, \x1b[97mDay {}\x1b[0m\n", DAY_NUM);
    match run() {
        Err(e) => println!("\x1b[91mError:\x1b[0m {}", e),
        Ok((s, g)) => {
            if let Some(s) = s { println!("🥈: \x1b[92m{}\x1b[0m", s); }
            if let Some(g) = g { println!("🥇: \x1b[92m{}\x1b[0m", g); }
        }
    }
}

fn run() -> Result<(Option<i64>, Option<i64>)> {
    let mut input_suffix: String = "ex".into();
    let mut verbose = false;

    let args = env::args().skip(1);
    for a in args {
        match a.as_str() {
            "-v" => verbose = true,
            _ => input_suffix = a,
        }
    }

    let file = format!("inputs/day{}-{}.txt", DAY_NUM, input_suffix);
    println!("Using input \x1b[36m{}\x1b[0m...", file);
    let f = File::open(file)?;
    let reader = io::BufReader::new(f);

    for (row, line) in reader.lines().enumerate() 
    {
        if let Ok(line) = line {
            if verbose {
                println!("Row #{} was {} characters", row, line.len());
            }
        }
    }

    Ok((Some(0), Some(0)))
}