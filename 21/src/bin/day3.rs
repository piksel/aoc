use std::convert::TryInto;
use std::io::{self, prelude::*};
use std::fs::File;
use aoc21::{Result, parse_args, print_results, DayResults, Opts};
use simple_error::{bail};

const DAY_NUM: i32 = 3;

fn main() {
    println!("\x1b[32mAoC \x1b[36m21\x1b[0m, \x1b[97mDay {}\x1b[0m\n", DAY_NUM);
    print_results(run());
}

fn get_bit_stats<'a>(lines: impl Iterator<Item=&'a String>, opts: &Opts, default: Option<char>) -> Result<String> {
    let mut bit_stats = vec!(0_i64; 0);
    for (row, line) in lines.enumerate() 
    {
        if bit_stats.is_empty() {
            bit_stats = vec!(0; line.len())
        }
        for (i, c) in line.chars().enumerate() {
            bit_stats[i] += if c == '1' {1} else {-1};
        }

        if opts.verbose {
            println!("Item #\x1b[36m{:3}\x1b[0m, report: \x1b[36m{}\x1b[0m, gamma: \x1b[36m{:?}\x1b[0m", row, line, bit_stats);
        }
    }

    let gamma_bits: String = bit_stats.iter().enumerate().map(|(i, n)| match *n {
        n if n < 0 => Ok('0'),
        n if n > 0 => Ok('1'), 
        _ if default.is_some() => Ok(default.unwrap()),
        _ => bail!("equal amounts of 0 and 1 bits in gamma position {}", i),
    }).collect::<Result<Vec<char>>>()?.iter().collect();

    return Ok(gamma_bits);
}

fn run() -> Result<DayResults> {
    let opts = parse_args()?;
    println!("{:?}", opts);
    let file = format!("inputs/day{}-{}.txt", DAY_NUM, opts.input_suffix);
    println!("Using input \x1b[36m{}\x1b[0m...", file);
    let reader = io::BufReader::new(File::open(file)?);

    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    let gamma_bits = get_bit_stats(lines.iter(), &opts, None)?;

    let gamma = i64::from_str_radix(gamma_bits.as_str(), 2)?;
    let mask = 2_i64.pow(gamma_bits.len().try_into().unwrap()) - 1;

    let epsilon = gamma ^ mask;
    let power = gamma * epsilon;

    if opts.verbose {
        println!("Gamma: \x1b[36m{}\x1b[0m (\x1b[36m{}\x1b[0m), mask: \x1b[36m{:b}\x1b[0m epsilon: \x1b[36m{}\x1b[0m, power: \x1b[36m{}\x1b[0m", 
            gamma, gamma_bits, mask, epsilon, power);
    }

    let mut candidates: Vec<&String> = lines.iter().collect();
    let mut most_common_bits = gamma_bits.clone();
    for i in 0..gamma_bits.len() {
        if candidates.len() == 1 {
            break;
        }

        candidates = candidates.drain(..).filter(|l| l.chars().nth(i) == most_common_bits.chars().nth(i)).collect();
        most_common_bits = get_bit_stats(candidates.clone().into_iter(), &opts, Some('1'))?;

        if opts.verbose {
            println!("Iter: \x1b[36m{}\x1b[0m, mcb: \x1b[36m{}\x1b[0m,  candidates: \x1b[36m{:?}\x1b[0m", i, most_common_bits, candidates.len());
        }
    }

    let oxygengen = i64::from_str_radix(candidates[0].as_str(), 2)?;

    candidates = lines.iter().collect();
    most_common_bits = gamma_bits.clone();
    for i in 0..gamma_bits.len() {
        if candidates.len() == 1 {
            break;
        }
        candidates = candidates.drain(..).filter(|l| l.chars().nth(i) != most_common_bits.chars().nth(i)).collect();
        most_common_bits = get_bit_stats(candidates.clone().into_iter(), &opts, Some('1'))?;


        if opts.verbose {
            println!("Iter: \x1b[36m{}\x1b[0m, mcb: \x1b[36m{}\x1b[0m, candidates: \x1b[36m{:?}\x1b[0m", i, most_common_bits, candidates.len());
        }
    }
    let co2scrub = i64::from_str_radix(candidates[0].as_str(), 2)?;


    let life = oxygengen * co2scrub;
    if opts.verbose {
        println!("Oxygen Generator: \x1b[36m{}\x1b[0m CO2 Scrubbing: \x1b[36m{}\x1b[0m, Life Support: \x1b[36m{}\x1b[0m", oxygengen, co2scrub, life);
    }


    Ok((Some(power), Some(life)))
}