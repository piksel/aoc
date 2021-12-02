use std::io::{self, prelude::*};
use std::fs::File;
use simple_error::bail;
use aoc21::{Result, parse_args, print_results};

const DAY_NUM: i32 = 2;

fn main() {
    println!("\x1b[32mAoC \x1b[36m21\x1b[0m, \x1b[97mDay {}\x1b[0m\n", DAY_NUM);
    print_results(run());
}

fn run() -> Result<(Option<i64>, Option<i64>)> {
    let opts = parse_args()?;
    let file = format!("inputs/day{}-{}.txt", DAY_NUM, opts.input_suffix);
    println!("Using input \x1b[36m{}\x1b[0m...", file);
    let reader = io::BufReader::new(File::open(file)?);

    let mut pos = (0, 0, 0);
    for (row, line) in reader.lines().enumerate()
    {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();
            let dir = parts.next().unwrap();
            let count: i64 = parts.next().unwrap().parse()?;

            match dir {
                "up" => pos.1 -= count,
                "down" => pos.1 += count,
                "forward" => {
                    pos.0 += count;
                    if opts.gold { pos.2 += pos.1 * count }
                },
                _ => bail!("invalid direction \"{}\" on row {}", dir, row)
            }

            if opts.verbose {
                println!("Row #\x1b[36m{}\x1b[0m, \x1b[36m{}\x1b[0m steps \x1b[36m{:<7}\x1b[0m => [\x1b[36m{:4}\x1b[0m, \x1b[36m{:4}\x1b[0m, \x1b[36m{:4}\x1b[0m]", 
                    row, count, dir, pos.0, pos.1, pos.2);
            }
        }
    }

    if !opts.gold {
        Ok((Some(pos.0 * pos.1), None))
    } else {
        Ok((None, Some(pos.0 * pos.2)))
    }
}