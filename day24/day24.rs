use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

struct Bridge {
    strength: u32,
    length: u32,
}

fn strongest_bridge(
    port: u32,
    strength: u32,
    length: u32,
    components: &Vec<(u32, u32)>,
    used: &mut HashSet<usize>,
) -> Vec<Bridge> {
    let mut bridges = vec![];

    for (i, c) in components.iter().enumerate() {
        if (c.0 == port || c.1 == port) && !used.contains(&i) {
            used.insert(i);
            bridges.append(&mut strongest_bridge(
                if c.0 == port { c.1 } else { c.0 },
                strength + c.0 + c.1,
                length + 1,
                components,
                used,
            ));
            used.remove(&i);
        }
    }

    bridges.push(Bridge { strength, length });

    bridges
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let components: Vec<(u32, u32)> = reader
        .lines()
        .map(|line| {
            let lines = line.unwrap();
            let ports: Vec<u32> = lines
                .split('/')
                .map(|port| port.parse().expect("invalid input"))
                .collect();
            assert_eq!(ports.len(), 2);
            (ports[0], ports[1])
        })
        .collect();

    let bridges = strongest_bridge(0, 0, 0, &components, &mut HashSet::new());

    println!(
        "part 1: {}",
        bridges.iter().max_by_key(|b| b.strength).unwrap().strength
    );
    println!(
        "part 2: {}",
        bridges
            .iter()
            .max_by(|a, b| {
                match a.length.cmp(&b.length) {
                    Ordering::Equal => a.strength.cmp(&b.strength),
                    cmp => cmp,
                }
            })
            .unwrap()
            .strength
    );
}
