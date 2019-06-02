use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let reader = BufReader::new(io::stdin());
    let list: Vec<Vec<usize>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" <-> ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|dst| dst.parse().unwrap())
                .collect()
        })
        .collect();

    let mut set = HashSet::new();
    let mut group_sizes = Vec::new();
    for i in 0..list.len() {
        if set.contains(&i) {
            continue;
        }

        let mut queue = VecDeque::new();
        set.insert(i);
        queue.push_back(i);
        let mut cnt = 0;

        while let Some(src) = queue.pop_front() {
            for &dst in &list[src] {
                if !set.contains(&dst) {
                    set.insert(dst);
                    queue.push_back(dst);
                }
            }
            cnt += 1;
        }

        group_sizes.push(cnt);
    }

    println!("part 1: {}", group_sizes[0]);
    println!("part 2: {}", group_sizes.len());
}
