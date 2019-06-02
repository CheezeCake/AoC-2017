use std::collections::HashSet;
use std::env;

fn knot_hash(lengths: &Vec<u8>) -> Vec<u8> {
    let mut list: Vec<u8> = (0..=255).collect();
    let len = list.len();
    let mut pos = 0usize;
    let mut skip_size = 0;

    for _i in 0..64 {
        for &length in lengths {
            let length = length as usize;
            for i in 0..(length / 2) as usize {
                let start = (pos + i) % len;
                let end = (pos + length - i - 1) % len;
                let tmp = list[start];
                list[start] = list[end];
                list[end] = tmp;
            }

            pos = (pos + length + skip_size) % len;
            skip_size += 1;
        }
    }

    list
}

fn knot_hash_str(s: &str) -> Vec<u8> {
    let mut lengths: Vec<u8> = s.as_bytes().to_vec();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let hash = knot_hash(&lengths);
    let dense_hash: Vec<u8> = (0..16)
        .map(|n| hash.iter().skip(n * 16).take(16).fold(0, |acc, b| acc ^ b))
        .collect();

    dense_hash
}

fn visit(grid: &Vec<Vec<bool>>, row: i32, column: i32, set: &mut HashSet<(i32, i32)>) {
    set.insert((row, column));

    for (dr, dc) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let nr = row + dr;
        let nc = column + dc;
        if nr >= 0
            && (nr as usize) < grid.len()
            && nc >= 0
            && (nc as usize) < grid.len()
            && grid[nr as usize][nc as usize]
            && !set.contains(&(nr, nc))
        {
            visit(&grid, nr, nc, set);
        }
    }
}

fn count_groups(grid: &Vec<Vec<bool>>) -> usize {
    let mut set = HashSet::new();
    let mut groups = 0usize;

    for row in 0..128 as i32 {
        for column in 0..128 as i32 {
            if grid[row as usize][column as usize] && !set.contains(&(row, column)) {
                visit(&grid, row, column, &mut set);
                groups += 1;
            }
        }
    }

    groups
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let grid: Vec<Vec<bool>> = (0..128)
        .map(|row| {
            let hash = knot_hash_str(&format!("{}-{}", input, row));
            hash.iter()
                .map(|x| {
                    (0..8)
                        .rev()
                        .map(|shift| (x >> shift) as i32 & 1 == 1)
                        .collect::<Vec<bool>>()
                })
                .flatten()
                .collect()
        })
        .collect();

    println!(
        "part 1: {}",
        grid.iter().flatten().fold(0, |sum, &x| sum + x as usize)
    );
    println!("part 2: {}", count_groups(&grid));
}
