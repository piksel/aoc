use std::io::{self, prelude::*};
use std::fs::File;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const DAY_NUM: i32 = 1;

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
                if verbose {
                    println!("{}: {:#?} WCurr: {}, WPrev: {} {}{}", 
                        row, msmt, curr_win, last_win, 
                        if  inc {"🥈"} else {"  "}, 
                        if winc {"🥇"} else {"  "}
                    );
                }
                last_win = curr_win;
            } else {
                if verbose { println!("{}: {:#?} Not enough data", row, msmt); }
            }
            

            if inc { inc_count += 1; }
            if winc { inc_win_count += 1; }
            last_msmts[1] = last_msmts[0];
            last_msmts[0] = msmt;
        }
    }

    Ok((Some(inc_count), Some(inc_win_count)))
}