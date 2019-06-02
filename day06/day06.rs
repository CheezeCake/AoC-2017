use std::collections::HashMap;
use std::io;

fn max_index(banks: &Vec<u32>) -> usize {
    let mut idx = 0usize;
    let mut max = 0u32;

    for (i, &n) in banks.iter().enumerate() {
        if n > max {
            idx = i;
            max = n;
        }
    }

    idx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");

    let mut banks: Vec<u32> = input
        .split_whitespace()
        .map(|blocks| blocks.parse().unwrap())
        .collect();
    let banks_size = banks.len();

    let mut cycles = 0usize;
    let mut seen: HashMap<Vec<u32>, usize> = HashMap::new();

    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), cycles);

        let mut idx = max_index(&banks);
        let n = banks[idx];
        banks[idx] = 0;

        for _i in 0..n {
            idx += 1;
            banks[idx % banks_size] += 1;
        }

        cycles += 1
    }

    println!("part 1: {}", cycles);
    println!("part 2: {}", cycles - seen.get(&banks).unwrap());
}
