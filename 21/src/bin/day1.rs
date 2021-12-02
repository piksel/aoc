use std::io::{self, prelude::*};
use std::fs::File;
use aoc21::{Result, parse_args, print_results};

const DAY_NUM: i32 = 1;

fn main() {
    println!("\x1b[32mAoC \x1b[36m21\x1b[0m, \x1b[97mDay {}\x1b[0m\n", DAY_NUM);
    print_results(run());
}

fn run() -> Result<(Option<i64>, Option<i64>)> {
    let opts = parse_args()?;
    let file = format!("inputs/day{}-{}.txt", DAY_NUM, opts.input_suffix);
    println!("Using input \x1b[36m{}\x1b[0m...", file);
    let reader = io::BufReader::new(File::open(file)?);

    let mut last_msmts = vec!(i64::MAX, 2);
    let mut last_win = i64::MAX;
    let mut inc_count = 0;
    let mut inc_win_count = 0;
    for (row, line) in reader.lines().enumerate() 
    {
        if let Ok(line) = line {
            let msmt: i64 = line.parse()?;

            let inc = msmt > last_msmts[0];
            let mut winc = false;
            if last_msmts[0] < i64::MAX && last_msmts[1] < i64::MAX {
                let curr_win = last_msmts[1] + last_msmts[0] + msmt;
                winc = curr_win > last_win;
                if opts.verbose {
                    println!("{}: {:#?} WCurr: {}, WPrev: {} {}{}", 
                        row, msmt, curr_win, last_win, 
                        if  inc {"🥈"} else {"  "}, 
                        if winc {"🥇"} else {"  "}
                    );
                }
                last_win = curr_win;
            } else {
                if opts.verbose { println!("{}: {:#?} Not enough data", row, msmt); }
            }
            

            if inc { inc_count += 1; }
            if winc { inc_win_count += 1; }
            last_msmts[1] = last_msmts[0];
            last_msmts[0] = msmt;
        }
    }

    Ok((Some(inc_count), Some(inc_win_count)))
}