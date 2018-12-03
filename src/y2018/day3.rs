extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

struct Point {
    id: u32,
    x:  u32, y:  u32,
    x1: u32, y1: u32,
}

pub fn run(vs: Vec<String>) {
    let (star1, star2) = star1_star2(vs);
    println!("{}\n{}", star1, star2);
}

fn star1_star2(vs: Vec<String>) -> (usize, u32) {
    let mut points: Vec<Point> = Vec::new();
    let mut mapa = HashMap::new();
    let mut claim = HashMap::new();
    let mut all_points = HashSet::new();
    let mut cross_points = HashSet::new();
    for s in vs.iter() {
        let mut point: Point = parse_point(s.to_string());
        points.push(point);
    }

    for point in points.iter() {
        all_points.insert(point.id);
        for x in point.x..point.x + point.x1 {
            for y in point.y..point.y + point.y1 {
                *mapa.entry((x, y)).or_insert(0) += 1;
                if claim.contains_key(&(x, y)) {
                    cross_points.insert(point.id);
                    cross_points.insert(claim[&(x, y)]);
                } else {
                    claim.insert((x, y), point.id);
                }
            }
        }
    }

    return (
        mapa.values().filter(|p| **p > 1).count(),
        *all_points.difference(&cross_points).next().unwrap()
    )
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
