use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    const N: u32 = 361527;
    let mut x = 0i32;
    let mut y = 0i32;
    let mut square_size = 1;
    let mut i = 1;
    let mut v: Option<u32> = None;
    let mut vals = HashMap::new();
    vals.insert((0, 0), 1);

    while i != N {
        for steps in &[
            (Direction::Right, 1),
            (Direction::Up, square_size),
            (Direction::Left, square_size + 1),
            (Direction::Down, square_size + 1),
            (Direction::Right, square_size + 1),
        ] {
            for _i in 0..steps.1 {
                if i == N {
                    break;
                }

                match steps.0 {
                    Direction::Up => y -= 1,
                    Direction::Down => y += 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }
                i += 1;

                if let None = v {
                    let mut val = 0u32;
                    for p in &[
                        (x, y - 1),
                        (x + 1, y - 1),
                        (x + 1, y),
                        (x + 1, y + 1),
                        (x, y + 1),
                        (x - 1, y + 1),
                        (x - 1, y),
                        (x - 1, y - 1),
                    ] {
                        val += vals.get(p).unwrap_or(&0);
                    }
                    if val > N {
                        v = Some(val);
                    } else {
                        vals.insert((x, y), val);
                    }
                }
            }
        }
        square_size += 2;
    }

    println!("part 1: {}", x.abs() + y.abs());
    println!("part 2: {}", v.unwrap());
}
