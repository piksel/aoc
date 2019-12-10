use std::fs;
use std::i32;

pub fn part1() -> String {
    println!("Day 1 - Part 1");
    let filename = "day1/input.txt";

    // fuel_required(12);
    // fuel_required(14);
    // fuel_required(1969);
    // fuel_required(100756);

    let mut total_fuel = 0;

    print!("Reading input {}...", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    for line in contents.split_whitespace() {
        let mass = line.parse::<i32>()
            .expect("Ïncorrect float format");
        total_fuel += fuel_required(mass);
        
    }

    println!();
    
    return total_fuel.to_string();
    
}

#[allow(dead_code)]
pub fn part2() -> String {
    println!("Day 1 - Part 2");
    let filename = "day1/input.txt";

    let mut total_fuel = 0;

    print!("Reading input {}...", filename);
    let contents = fs::read_to_string(filename).unwrap();

    for line in contents.split_whitespace() {
        let mass = line.parse::<i32>().unwrap();

        total_fuel += meta_fuel_required(mass);
    }

    println!();
    return total_fuel.to_string();
    
}

fn fuel_required(mass: i32) -> i32 {
    
    // mass, divide by three, round down, and subtract 2.
    return (((mass as f32 / 3.0) as f32).floor() as i32 - 2).max(0);

}

fn meta_fuel_required(mass: i32) -> i32 {
    let fuel = fuel_required(mass);
    if fuel > 0 {
        let meta_fuel = meta_fuel_required(fuel);
        
        return fuel + meta_fuel ;
    } else {
        return fuel;
    }
}
