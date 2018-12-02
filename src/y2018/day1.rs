use std::collections::HashSet;

pub fn run(vs: Vec<String>) {
    println!("{}", star1(vs.clone()));
    println!("{}", star2(vs.clone()));
}

fn star1(vs: Vec<String>) -> isize {
    let mut sum: isize = 0;
    vs.iter().for_each(|x| sum += x.parse::<isize>().unwrap());
    return sum;
}

fn star2(vs: Vec<String>) -> isize {
    let mut sum: isize = 0isize;
    let cs: Vec<isize> = vs.iter().map(|x| x.parse::<isize>().unwrap()).collect();
    let mut hash_set = HashSet::new();
    //hashSet.insert(0isize);

    loop {
        for c in &cs {
            sum += c;
            if hash_set.contains(&sum) {
                return sum;
            } else {
                hash_set.insert(sum);
            }
        }
    }
}
