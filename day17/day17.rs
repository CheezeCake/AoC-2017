use std::env;

struct Node {
    value: usize,
    next: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let steps: usize = args[1].parse().unwrap();

    let mut buffer = Vec::new();
    buffer.reserve_exact(2018);
    buffer.push(Node { value: 0, next: 0 });
    let mut pos = 0usize;

    for n in 1..=2017 {
        for _i in 0..steps {
            pos = buffer[pos].next;
        }

        let next = buffer[pos].next;
        buffer.push(Node { value: n, next });
        buffer[pos].next = buffer.len() - 1;
        pos = buffer.len() - 1;
    }

    let next = buffer[pos].next;
    println!("part 1: {}", buffer[next].value);

    let mut afert_0 = 0;
    pos = 0;
    for n in 1..=50_000_000 {
        pos = ((pos + steps) % n) + 1;
        if pos == 1 {
            afert_0 = n;
        }
    }
    println!("part 2: {}", afert_0);
}
