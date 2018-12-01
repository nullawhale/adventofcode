extern crate regex;
use self::regex::Regex;

pub fn run(vs: Vec<String>) {

    println!("{}", star1(vs.clone()));
}

fn star1(vs: Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in vs.iter() {
        let re = Regex::new(r"\s+").unwrap();
        let qwe: Vec<&str> = re.split(line).collect();
        let qwer: Vec<usize> = qwe.iter().map(|x| x.parse().unwrap()).collect();

        sum += qwer.iter().max().unwrap() - qwer.iter().min().unwrap();
    }

    return sum;
}
