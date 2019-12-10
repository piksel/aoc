use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    
    println!("Day3: ");

    let mut f = File::open("day3.input.txt")?;
    let mut reader = BufReader::new(f);

    for (num, line) in file.lines().enumerate() 
    {
        let parts = line.unwrap().split_whitespace();
        let id = parts.next();
        let at = parts.next();
        let coords = parts.next();
        let size = parts.next();
    }
    

}