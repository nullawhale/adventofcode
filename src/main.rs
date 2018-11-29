mod y2017;

use std::io::Read;

fn main() {
    y2017::day1::run(read_vec_for_day(1));
}

fn read_vec_for_day(day: u8) -> Vec<u8> {
    let mut buffer = String::new();
    let path = format!("./input/day{}.txt", day);
    let mut input = std::fs::File::open(&path).expect("БЛА-БЛА-БЛА");
    input.read_to_string(&mut buffer).expect("БЛА_БЛА_БЛА",);

    let b = buffer.as_bytes();
    let mut vec: Vec<u8> = b.into_iter().map(|x| x - '0' as u8).collect();

    vec.pop();

    return vec;
}
