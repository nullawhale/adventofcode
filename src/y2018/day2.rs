use std::collections::HashMap;

pub fn run(vs: Vec<String>) {
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
    println!("{} * {} = {}", twice, thrice, twice*thrice);
}
