use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut pos: (i32, i32, i32) = (0, 0, 0);
    let distances: Vec<i32> = input
        .trim()
        .split(',')
        .map(|dir| {
            let (x, y, z) = pos;
            pos = match dir {
                "n" => (x, y + 1, z - 1),
                "ne" => (x + 1, y, z - 1),
                "se" => (x + 1, y - 1, z),
                "s" => (x, y - 1, z + 1),
                "sw" => (x - 1, y, z + 1),
                "nw" => (x - 1, y + 1, z),
                _ => panic!(format!("invalid direction: {}", dir)),
            };
            (pos.0.abs() + pos.1.abs() + pos.2.abs()) / 2
        })
        .collect();

    println!("part 1: {}", distances.last().unwrap());
    println!("part 2: {}", distances.iter().max().unwrap());
}
