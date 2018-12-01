mod y2017;
mod y2018;

use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let year: u32 = args[1].parse().unwrap();
    let day: u8   = args[2].parse().unwrap();

    if year == 2017 {
        match day {
            1 => y2017::day1::run(read_vec_for_day(year, day)),
            2 => y2017::day2::run(read_lines(year, day)),

            _ => println!("\nSomething wrong, try to enter another day"),
        }
    }
    if year == 2018 {
        match day {
            1 => y2018::day1::run(read_lines(year, day)),

            _ => println!("\nSomething wrong, try to enter another day"),
        }
    }
}

fn read_vec_for_day(year: u32, day: u8) -> Vec<u8> {
    let mut buffer = String::new();
    let path = format!("./input/{}/day{}.txt", year, day);
    let mut input = File::open(&path).expect("Can't open file");
    input.read_to_string(&mut buffer).expect("Can't read file",);

    let b = buffer.as_bytes();
    let mut vec: Vec<u8> = b.into_iter().map(|x| x - '0' as u8).collect();

    vec.pop();

    return vec;
}

fn read_lines(year: u32, day: u8) -> Vec<String> {
    let mut buffer: Vec<String> = Vec::new();

    let path = format!("./input/{}/day{}.txt", year, day);
    let input = File::open(&path).expect("Can't open file");
    let input = BufReader::new(input);
    for line in input.lines() {
        //println!("{}", line.unwrap());
        buffer.push(line.unwrap());
    }
    return buffer;
}
