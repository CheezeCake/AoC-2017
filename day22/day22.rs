use std::collections::HashMap;
use std::io;

#[derive(Clone)]
enum Node {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn read_grid() -> Grid {
    let mut line = String::new();
    let mut grid = HashMap::new();
    let mut y = 0;

    loop {
        if let Ok(0) = io::stdin().read_line(&mut line) {
            break;
        }

        let mut x = 0;
        for &c in line.trim_end_matches('\n').as_bytes() {
            grid.insert(
                Point { x, y },
                match c {
                    b'.' => Node::Clean,
                    b'#' => Node::Infected,
                    _ => panic!(format!("Invalid character '{}'", c)),
                },
            );
            x += 1;
        }
        y += 1;
        line.clear();
    }

    grid
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn turn_right(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn turn_left(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    }
}

fn move_forward(p: &Point, d: &Direction) -> Point {
    let x = p.x;
    let y = p.y;
    match d {
        Direction::Up => Point { x, y: y - 1 },
        Direction::Right => Point { x: x + 1, y },
        Direction::Down => Point { x, y: y + 1 },
        Direction::Left => Point { x: x - 1, y },
    }
}

type Grid = HashMap<Point, Node>;

fn burst(node: &mut Node, direction: &mut Direction) {
    *node = if let Node::Infected = node {
        *direction = turn_right(direction);
        Node::Clean
    } else {
        *direction = turn_left(direction);
        Node::Infected
    };
}

fn evolved_burst(node: &mut Node, direction: &mut Direction) {
    *node = match node {
        Node::Clean => {
            *direction = turn_left(&direction);
            Node::Weakened
        }
        Node::Infected => {
            *direction = turn_right(direction);
            Node::Flagged
        }
        Node::Weakened => Node::Infected,
        Node::Flagged => {
            *direction = turn_right(&turn_right(direction));
            Node::Clean
        }
    };
}

fn count_infections(mut grid: Grid, bursts: usize, burst: &Fn(&mut Node, &mut Direction)) -> usize {
    let middle = grid.keys().max_by_key(|k| k.x).expect("empty grid").x / 2;
    let mut position = Point {
        x: middle,
        y: middle,
    };
    let mut direction = Direction::Up;
    let mut infections = 0;

    for _ in 0..bursts {
        let node = grid.entry(position).or_insert(Node::Clean);
        burst(node, &mut direction);
        if let Node::Infected = node {
            infections += 1;
        }
        position = move_forward(&position, &direction);
    }

    infections
}

fn main() {
    let grid = read_grid();
    println!("part 1: {}", count_infections(grid.clone(), 10_000, &burst));
    println!(
        "part 2: {}",
        count_infections(grid.clone(), 10_000_000, &evolved_burst)
    );
}
