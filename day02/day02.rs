use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn divisible_pair_result(values: &Vec<i32>) -> Option<i32> {
    for (i, x) in values.iter().enumerate() {
        for (j, y) in values.iter().enumerate() {
            if i != j && x % y == 0 {
                return Some(x / y);
            }
        }
    }
    None
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let lines: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(char::is_whitespace)
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let sum = |f: &Fn(&Vec<i32>) -> i32| lines.iter().map(f).sum::<i32>();

    println!(
        "part 1: {}",
        sum(&|line| line.iter().max().unwrap() - line.iter().min().unwrap())
    );
    println!(
        "part 2: {}",
        sum(&|line| divisible_pair_result(line).unwrap())
    );
}
