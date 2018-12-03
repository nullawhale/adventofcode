use std::fmt;
extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    id: u32,
    x:  u32,
    y:  u32,
    x1: u32,
    y1: u32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn run(vs: Vec<String>) {
    println!("{}", star1(vs));
}

fn star1(vs: Vec<String>) -> usize {
    let mut points: Vec<Point> = Vec::new();
    let mut mapa = HashMap::new();
    for s in vs.iter() {
        let mut point: Point = parse_point(s.to_string());
        points.push(point);
        //println!("{:?}", point.x);
        //println!("{} {} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);
        //vs.iter().for_each(|s| println!("{}", s));
    }

    for point in points.iter() {
        for x in point.x..point.x + point.x1 {
            for y in point.y..point.y + point.y1 {
                *mapa.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    return mapa.values().filter(|p| **p > 1).count();
}

fn parse_point(s: String) -> Point {
    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let cap = re.captures(&s).unwrap();
    let point = Point {
        id: String::from(&cap[1]).parse::<u32>().unwrap(),
        x:  String::from(&cap[2]).parse::<u32>().unwrap(),
        y:  String::from(&cap[3]).parse::<u32>().unwrap(),
        x1: String::from(&cap[4]).parse::<u32>().unwrap(),
        y1: String::from(&cap[5]).parse::<u32>().unwrap()
    };

    return point;
}

// #1 @ 1,3: 4x4
// #2 @ 3,1: 4x4
// #3 @ 5,5: 2x2
