use std::convert::TryInto;
use std::io::{self, prelude::*};
use std::fs::File;
use aoc21::{Result, DayResults, parse_args, print_results, Opts};
use simple_error::bail;

const DAY_NUM: i32 = 4;

fn main() {
    println!("\x1b[32mAoC \x1b[36m21\x1b[0m, \x1b[97mDay {}\x1b[0m\n", DAY_NUM);
    print_results(run());
}

fn run() -> Result<DayResults> {
    let opts = parse_args()?;
    let file = format!("inputs/day{}-{}.txt", DAY_NUM, opts.input_suffix);
    println!("Using input \x1b[36m{}\x1b[0m...", file);
    let reader = io::BufReader::new(File::open(file)?);

    let lines = &mut reader.lines();
    let numbers: Vec<i64> = lines.next().unwrap()?.split(',').map(|n| i64::from_str_radix(n, 10).unwrap()).collect();

    println!("Numbers: \x1b[36m{:?}\x1b[0m", numbers);
    println!();

    let mut boards: Vec<[[i64; 5]; 5]> = Vec::new();
    let mut board_draws: Vec<[[bool; 5]; 5]> = Vec::new();
    // Skip blank line before board
    while let Some(Ok(sep_line)) = lines.next() {
        if !sep_line.is_empty() { bail!("Missing blank line before board") }
        boards.push(lines.take(5).map(|row| 
                row.unwrap().split_whitespace().map(|n| 
                    i64::from_str_radix(n, 10)
                        .unwrap_or_else(|e| panic!("Invalid board number: {}", e))
                ).collect::<Vec<i64>>().try_into()
                    .unwrap_or_else(|v: Vec<i64>| panic!("Invalid board width: {}", v.len()))
            ).collect::<Vec<[i64; 5]>>().try_into()
                .unwrap_or_else(|v: Vec<[i64; 5]>| panic!("Invalid board height: {}", v.len())));
        board_draws.push([[false; 5];5]);
    }

    let mut boards_have_won = vec![false; boards.len()];
    let mut first_winner_score = None;
    let mut last_winner_score = None;
    let mut winners = 0_usize;
    
    for (round, drawn) in numbers.into_iter().enumerate() {
        // Clear screen:
        //print!("\x1b[2J");
        println!("Round \x1b[36m{}\x1b[0m, the drawn number is \x1b[36m{}\x1b[0m!", round, drawn);
        println!();
        for winner in update_boards(&opts, &boards, &mut board_draws, drawn, &mut boards_have_won) {
            boards_have_won[winner] = true;

            println!("BINGO at board \x1b[36m{}\x1b[0m!", winner+1);
            let unmarked: i64 = boards[winner].iter().zip(board_draws[winner])
                .map(|(rn, rm)| rn.iter().zip(rm).collect::<Vec<(&i64, bool)>>())
                .flatten()
                .map(|(num, marked)| if marked {0} else {*num})
                .sum();
            let score = unmarked * drawn;
            println!("Unmarked score: \x1b[36m{}\x1b[0m, score product: \x1b[36m{}\x1b[0m", unmarked, score);
            println!();

            winners += 1;

            if winners == 1 {
                first_winner_score = Some(score);
            }
            last_winner_score = Some(score);
        }
        if winners == boards.len() {
            // Everyone wins! Yay!
            break;
        }
        
        // Delays:
        //thread::sleep(Duration::from_millis(100));
        //let _ = stdin().read(&mut [0u8]).unwrap();

        println!("Winners: \x1b[36m{}\x1b[0m of \x1b[36m{}\x1b[0m", winners, boards.len());

        println!();
    }

    Ok((first_winner_score, last_winner_score))
}

fn update_boards(opts: &Opts, boards: &[[[i64; 5]; 5]], board_draws: &mut [[[bool; 5]; 5]], drawn_number: i64, boards_won: &Vec<bool>) -> Vec<usize> {
    let mut winners = Vec::new();
    for (bi, (board, draws)) in boards.iter().zip(board_draws).enumerate() {
        if opts.verbose {println!("Board \x1b[36m{}\x1b[0m", bi + 1);}
        let mut board_has_won = boards_won[bi];
        let mut bingo_cols = [true; 5];
        for (row_nums, row_draws) in board.iter().zip(draws) {
            let mut bingo_row = true;
            for (ci, (num, drawn)) in row_nums.iter().zip(row_draws).enumerate() {
                *drawn |= *num == drawn_number;
                bingo_row &= *drawn;
                bingo_cols[ci] &= *drawn;
                if opts.verbose {
                    if *drawn {
                        print!("\x1b[92m{:2}\x1b[0m ", num);
                    } else {
                        print!("\x1b[90m{:2}\x1b[0m ", num);
                    }
                }
            }
            if !board_has_won && bingo_row { winners.push(bi); board_has_won = true; }
            if opts.verbose {println!();}
        }
        if !board_has_won && bingo_cols.iter().any(|b| *b) { winners.push(bi); }
        if opts.verbose {println!();}
    }
    return winners;
}