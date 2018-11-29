pub fn run(bytes: Vec<u8>) {

    println!("{}", star_1(bytes.clone()));
    println!("{}", star_2(bytes.clone()));
}

pub fn star_1(mut bytes: Vec<u8>) -> usize {
    let mut sum: usize = 0;
    let first = bytes[0];

    let mut b: Vec<u8> = Vec::new();
    b.append(&mut bytes);
    b.push(first);
    let len = b.len();

    for i in 0..len-1 {
        if (b[i] == b[i + 1]) {
            sum += b[i] as usize;
        }
    }

    return sum;
}

pub fn star_2(mut b: Vec<u8>) -> usize {
    let mut sum: usize = 0;
    let len = b.len()/2;
    let b2: Vec<u8> = b.split_off(len);

    b.iter().zip(b2.iter())
        .filter(|(x, y)| x == y)
        .for_each(|(x, y)| sum += (x+y) as usize);

    return sum;
}
