use std::collections::HashMap;

pub fn run(vs: Vec<String>) {
    let (twice, thrice) = star1(vs.clone());
    println!("{} * {} = {}", twice, thrice, twice*thrice);
    println!("{}", star2(vs.clone()));
}

fn star1(vs: Vec<String>) -> (usize, usize) {
    let (mut twice, mut thrice) = (0, 0);
    for line in vs.iter() {
        let mut counts = HashMap::new();
        //let l:Vec<char> = line.chars().collect();
        for ch in line.chars() {
            let e = counts.entry(ch).or_insert(0);
            *e += 1;
        }
        if counts.values().any(|&x| x == 2) { twice  += 1; }
        if counts.values().any(|&x| x == 3) { thrice += 1; }
    }

    return (twice, thrice);
}

fn star2(vs: Vec<String>) -> String {
    for one in &vs {
        for another in &vs {
            if one.len() == another.len() {
                if one != another {
                    let mut diff = 0;
                    let mut result = String::new();
                    for x in one.chars().zip(another.chars()) {
                        if x.0 != x.1 { diff += 1; }
                        else { result.push(x.0); }
                    }
                    if diff == 1 { return result; }
                }
            }
        }
    }
    unreachable!();
}
