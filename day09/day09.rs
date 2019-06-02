use std::io;
use std::slice;

fn parse_garbage(input: &mut slice::Iter<'_, u8>) -> Option<u32> {
    let mut count = 0;

    loop {
        let &c = input.next()?;

        if c == b'!' {
            input.next()?;
        } else if c == b'>' {
            return Some(count);
        } else {
            count += 1;
        }
    }
}

fn parse_group(input: &mut slice::Iter<'_, u8>, cur_score: u32) -> Option<(u32, u32)> {
    let mut score = cur_score;
    let mut garbage_count = 0;

    loop {
        let &c = input.next()?;

        if c == b'{' {
            let (s, cnt) = parse_group(input, cur_score + 1)?;
            score += s;
            garbage_count += cnt;
        } else if c == b'<' {
            garbage_count += parse_garbage(input)?;
        } else if c == b'}' {
            return Some((score, garbage_count));
        } else if c != b',' {
            panic!("expected ','");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().as_bytes().iter();

    assert_eq!(input.next().unwrap(), &b'{');

    let (score, garbage_count) = parse_group(&mut input, 1).unwrap();

    println!("part 1: {}", score);
    println!("part 2: {}", garbage_count);
}
