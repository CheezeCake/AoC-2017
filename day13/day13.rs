use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn trip(ranges: &HashMap<u32, u32>, delay: u32) -> Vec<u32> {
    let mut catches = Vec::new();

    for (&depth, &range) in ranges {
        let picosec = depth + delay;
        let mut scanner = picosec % (2 * (range - 1));
        if scanner >= range - 1 {
            scanner = range - 1 - (scanner % (range - 1));
        }
        if scanner == 0 {
            catches.push(depth);
        }
    }

    catches
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let ranges: HashMap<u32, u32> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut values = line.split(": ").map(|val| val.parse().unwrap());
            (values.next().unwrap(), values.next().unwrap())
        })
        .collect();

    println!(
        "part 1: {}",
        trip(&ranges, 0)
            .iter()
            .fold(0, |acc, depth| acc + depth * ranges[depth])
    );

    for delay in 1.. {
        if trip(&ranges, delay).len() == 0 {
            println!("part 2: {}", delay);
            return;
        }
    }
}
