use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn count_matches(
    initial_a: u64,
    a_divisor: u64,
    initial_b: u64,
    b_divisor: u64,
    pairs: usize,
) -> usize {
    let mut a = initial_a;
    let mut b = initial_b;
    let mut cnt = 0;

    for _i in 0..pairs {
        loop {
            a = (a * 16807) % 2147483647;
            if a % a_divisor == 0 {
                break;
            }
        }
        loop {
            b = (b * 48271) % 2147483647;
            if b % b_divisor == 0 {
                break;
            }
        }
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
