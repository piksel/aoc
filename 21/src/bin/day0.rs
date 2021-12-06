use std::io::{self, prelude::*};
use std::fs::File;
use aoc21::{Result, DayResults, parse_args, print_results};

const DAY_NUM: i32 = 0;

fn main() {
    println!("\x1b[32mAoC \x1b[36m21\x1b[0m, \x1b[97mDay {}\x1b[0m\n", DAY_NUM);
    print_results(run());
}

fn run() -> Result<DayResults> {
    let opts = parse_args()?;
    let file = format!("inputs/day{}-{}.txt", DAY_NUM, opts.input_suffix);
    println!("Using input \x1b[36m{}\x1b[0m...", file);
    let reader = io::BufReader::new(File::open(file)?);

    for (row, line) in reader.lines().enumerate() 
    {
        if let Ok(line) = line {
            if opts.verbose {
                println!("Row #{} was {} characters", row, line.len());
            }
        }
    }

    Ok((Some(0), Some(0)))
}