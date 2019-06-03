use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn generate(x: u64, factor: u64, divisor: u64) -> u64 {
    let mut x = x;
    loop {
        x = (x * factor) % 2147483647;
        if x % divisor == 0 {
            return x;
        }
    }
}

fn count_matches(a: u64, a_divisor: u64, b: u64, b_divisor: u64, pairs: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut cnt = 0;

    for _i in 0..pairs {
        a = generate(a, 16807, a_divisor);
        b = generate(b, 48271, b_divisor);
        if a & 0xffff == b & 0xffff {
            cnt += 1;
        }
    }

    cnt
}

fn main() {
    let input: Vec<u64> = BufReader::new(io::stdin())
        .lines()
        .map(|line| line.unwrap().split(' ').last().unwrap().parse().unwrap())
        .collect();
    let a = input[0];
    let b = input[1];

    println!("part 1: {}", count_matches(a, 1, b, 1, 40_000_000));
    println!("part 2: {}", count_matches(a, 4, b, 8, 5_000_000));
}
