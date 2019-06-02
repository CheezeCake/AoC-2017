use std::io;

fn captcha(s: &str, step: usize) -> u32 {
    let bytes = s.as_bytes();
    let len = s.len();
    let mut cnt = 0;

    for (i, c) in bytes.iter().enumerate() {
        if c == bytes.iter().nth((i + step) % len).unwrap() {
            cnt += (c - b'0') as u32;
        }
    }

    cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    println!("part 1: {}", captcha(input, 1));
    println!("part 2: {}", captcha(input, input.len() / 2));
}
