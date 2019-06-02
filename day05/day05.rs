use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn count_steps(jumps: &mut Vec<i32>, f: &Fn(&mut i32)) -> usize {
    let len = jumps.len() as i32;
    let mut steps = 0;
    let mut pc = 0i32;

    while pc >= 0 && pc < len {
        let jmp = &mut jumps[pc as usize];
        pc += *jmp;
        f(jmp);

        steps += 1;
    }

    steps
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let jumps: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!(
        "part 1: {}",
        count_steps(&mut jumps.clone(), &|jmp| *jmp += 1)
    );

    println!(
        "part 2: {}",
        count_steps(&mut jumps.clone(), &|jmp| {
            if *jmp >= 3 {
                *jmp -= 1;
            } else {
                *jmp += 1;
            }
        })
    );
}
