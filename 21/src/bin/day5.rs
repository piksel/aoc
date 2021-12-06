use std::convert::TryInto;
use std::io::{self, prelude::*};
use std::fs::File;
use aoc21::{Result, DayResults, parse_args, print_results};

const DAY_NUM: i32 = 5;
const MAP_SIZE: usize = 1000;

fn main() {
    println!("\x1b[32mAoC \x1b[36m21\x1b[0m, \x1b[97mDay {}\x1b[0m\n", DAY_NUM);
    print_results(run());
}

fn run() -> Result<DayResults> {
    let opts = parse_args()?;
    let file = format!("inputs/day{}-{}.txt", DAY_NUM, opts.input_suffix);
    println!("Using input \x1b[36m{}\x1b[0m...", file);
    let reader = io::BufReader::new(File::open(file)?);

    let mut overlap_map = [[0; MAP_SIZE]; MAP_SIZE];

    for (row, line) in reader.lines().enumerate() 
    {
        if let Ok(line) = line {
            let [[x1, y1], [x2, y2]]: [[usize; 2]; 2] = line.split(" -> ")
                .map(|c| c.split(',')
                    .map(|n| usize::from_str_radix(n, 10)
                        .unwrap_or_else(|e| panic!("Error parsing {:#?}: {}", n, e)))
                    .collect::<Vec<usize>>().try_into().unwrap())
                .collect::<Vec<[usize; 2]>>().try_into().unwrap();
            
            if opts.verbose {
                println!("Row #\x1b[36m{}\x1b[0m , \x1b[36m{}\x1b[0m,\x1b[36m{}\x1b[0m => \x1b[36m{}\x1b[0m,\x1b[36m{}\x1b[0m", row, x1, y1, x2, y2);
            }

            if x1 == x2 {
                for y in if y1 < y2 {y1..=y2} else {y2..=y1} {
                    overlap_map[y][x1] += 1;
                }
            } else if y1 == y2 {
                for x in if x1 < x2 {x1..=x2} else {x2..=x1} {
                    overlap_map[y1][x] += 1;
                }
            } else if opts.gold {
                let range_y = if y1 < y2 {y1..=y2} else {y2..=y1};
                let range_x = if x1 < x2 {x1..=x2} else {x2..=x1};

                for (y, x) in (range_y).zip(
                    if (y1 < y2) == (x1 < x2) { range_x.collect() }
                    else { range_x.rev().collect::<Vec<usize>>() }
                ) {
                    overlap_map[y][x] += 1;
                }
            }

            if opts.verbose {
                let overlaps = process_map(&opts, &overlap_map);
                println!("Dangerous overlaps so far: \x1b[36m{}\x1b[0m", overlaps);
            }

        }
    }

    let dangerous_overlaps = process_map(&opts, &overlap_map);

    if opts.gold {
        Ok((None, Some(dangerous_overlaps)))
    } else {
        Ok((Some(dangerous_overlaps), None))
    }
    
}

fn process_map(opts: &aoc21::Opts, depth_map: &[[usize; MAP_SIZE]; MAP_SIZE]) -> i64 {
    let mut dangerous_overlaps = 0;
    for (y, cols) in depth_map.iter().enumerate() {
        for (x, cell) in cols.iter().enumerate() {
            if cell >= &2 {dangerous_overlaps += 1}
            if !opts.verbose || x > 100 || y > 100 {continue}
            if cell == &0 {
                print!("\x1b[90m .\x1b[0m");
            } else {
                print!("\x1b[93m{:2}\x1b[0m", cell);
            }
            
        }
        if opts.verbose && y <= 100 {println!();}
    }
    return dangerous_overlaps;
}