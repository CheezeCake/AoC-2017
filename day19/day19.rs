use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let reader = BufReader::new(io::stdin());
    let map: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();

    let mut y = 0i32;
    let mut x = 0i32;
    let mut dir: (i32, i32) = (0, 1);
    for (i, &c) in map[y as usize].iter().enumerate() {
        if c == b'|' {
            x = i as i32;
        }
    }

    let mut letters = String::new();
    let mut cnt = 0;

    loop {
        let c = map[y as usize][x as usize];

        if c == b' ' {
            break;
        } else if c.is_ascii_alphabetic() {
            letters.push(c as char);
        } else if c == b'+' {
            let up_down = [(0, -1), (0, 1)];
            let left_right = [(-1, 0), (1, 0)];
            for &d in &if dir.0 == 0 { left_right } else { up_down } {
                if map[(y + d.1) as usize][(x + d.0) as usize] != b' ' {
                    dir = d;
                    break;
                }
            }
        }

        x += dir.0;
        y += dir.1;
        cnt += 1;
    }

    println!("part 1: {}", letters);
    println!("part 2: {}", cnt);
}
