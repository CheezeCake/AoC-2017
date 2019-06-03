use std::collections::HashSet;
use std::io;
use std::str;

struct DanceMove {
    cmd: u8,
    op1: u8,
    op2: u8,
}

impl DanceMove {
    fn new(cmd: u8, op1: u8, op2: u8) -> DanceMove {
        DanceMove { cmd, op1, op2 }
    }
}

fn swap(programs: &mut [u8; 16], indices: &mut [u8; 16], p1: usize, p2: usize) {
    let tmp = programs[p1];
    programs[p1] = programs[p2];
    programs[p2] = tmp;

    let tmp = indices[programs[p1] as usize];
    indices[programs[p1] as usize] = indices[programs[p2] as usize];
    indices[programs[p2] as usize] = tmp;
}

fn dance(programs: &mut [u8; 16], indices: &mut [u8; 16], end: &mut i32, moves: &Vec<DanceMove>) {
    moves.iter().for_each(|m| {
        if m.cmd == b's' {
            *end -= m.op1 as i32;
            if *end < 0 {
                *end = 16 + *end;
            }
        } else if m.cmd == b'x' {
            let p1 = ((m.op1 as i32 + *end) % 16) as usize;
            let p2 = ((m.op2 as i32 + *end) % 16) as usize;
            swap(programs, indices, p1, p2);
        } else if m.cmd == b'p' {
            let p1 = indices[(m.op1 - b'a') as usize] as usize;
            let p2 = indices[(m.op2 - b'a') as usize] as usize;
            swap(programs, indices, p1, p2);
        }
    });
}

fn print(prefix: &str, programs: &[u8; 16], end: usize) {
    print!("{}", prefix);
    for i in 0..16 {
        print!("{}", (programs[(end + i) % 16] + b'a') as char);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut programs: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut indices = programs.clone();
    let mut end: i32 = 16;

    let moves: Vec<DanceMove> = input
        .trim()
        .split(',')
        .map(|s| {
            let bytes = s.as_bytes();
            let cmd = bytes[0];
            let args = str::from_utf8(&bytes[1..]).unwrap();

            match cmd {
                b's' => DanceMove::new(cmd, args.parse().unwrap(), 0),
                b'x' => {
                    let positions: Vec<u8> = args.split('/').map(|x| x.parse().unwrap()).collect();
                    DanceMove::new(cmd, positions[0], positions[1])
                }
                b'p' => DanceMove::new(cmd, bytes[1], bytes[3]),
                _ => panic!("invalid move"),
            }
        })
        .collect();

    let mut seen = HashSet::new();
    seen.insert(programs);

    dance(&mut programs, &mut indices, &mut end, &moves);
    seen.insert(programs);
    print("part 1: ", &programs, end as usize);

    for i in 0.. {
        dance(&mut programs, &mut indices, &mut end, &moves);
        if seen.contains(&programs) {
            break;
        }
        seen.insert(programs);
    }
    for _i in 0..(1_000_000_000 % seen.len()) {
        dance(&mut programs, &mut indices, &mut end, &moves);
    }
    print("part 2: ", &programs, end as usize);
}
