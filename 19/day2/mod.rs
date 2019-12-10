use std::fs;

const filename: &'static str = "day2/input.txt";

pub fn part1() -> String {
    println!("Day 2 - Part 1");
    // run_intcodes("1,9,10,3,2,3,11,0,99,30,40,50");
    // run_intcodes("1,1,1,4,99,5,6,0,99");
    let contents = fs::read_to_string(filename).unwrap();
    let answer = run_intcodes(&contents, 12, 2);
    return answer.to_string();
}

pub fn part2() -> String {
    println!("Day 2 - Part 2");

    let target = 19690720;

    print!("Reading input {}...", filename);
    let contents = fs::read_to_string(filename).unwrap();

    for noun in 0..100 {
        for verb in 0..100 {
            let result = run_intcodes(&contents, noun, verb);
            if result == target {
                println!("\nFound the correct params! Noun: {}, Verb: {}", noun, verb);
                let answer = 100 * noun + verb;
                return answer.to_string();
            } else {
                // println!("Incorrect result! Expected {}, got {}. Let's try something else...", target, result);
            }
        }
    }

    return String::from("");
 
}

fn run_intcodes(contents: &str, noun: usize, verb: usize) -> usize {

    let mut buffer = contents.split(',')
    .map(|c| c.parse::<usize>().unwrap() )
    .collect::<Vec<usize>>();

    buffer[1] = noun;
    buffer[2] = verb;

    let mut cursor = 0;
    loop {
        let code = buffer[cursor];
            cursor += 1;
        if code == 99 {
            /*
            println!("\nReached the end! Dumping buffer:");

            for ic in buffer.iter() {
                print!("{},", ic);
            }
            println!("");
            */
            return buffer[0];
        } else {

            let src1 = buffer[cursor];
            cursor += 1;
            let src2 = buffer[cursor];
            cursor += 1;
            let dst = buffer[cursor];
            cursor += 1;

            let vsrc1 = buffer[src1];
            let vsrc2 = buffer[src2];

            if code == 1 {
                //println!("Add  [{}] {} and [{}] {} => {}", src1, vsrc1, src2, vsrc2, dst);
                buffer[dst] = vsrc1 + vsrc2;
            } else if code == 2 {
                //println!("Mult [{}] {} and [{}] {} => {}", src1, vsrc1, src2, vsrc2, dst);
                buffer[dst] = vsrc1 * vsrc2;

            } else {
                eprintln!("Invalid int code: {}", code);
            }
        }
    }
}
