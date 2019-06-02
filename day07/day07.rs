use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let name_weight: Vec<&str> = parts[0].split_whitespace().collect();

        let name = name_weight[0].to_string();
        let weight: u32 = name_weight[1]
            .trim_matches(|p| p == '(' || p == ')')
            .parse()?;
        let children: Vec<String> = if let Some(children) = parts.get(1) {
            children
                .split(", ")
                .map(|child| child.to_string())
                .collect()
        } else {
            Vec::new()
        };

        Ok(Program {
            name,
            weight,
            children,
        })
    }
}

fn total_weight(programs: &HashMap<String, Program>, name: &String) -> Result<u32, u32> {
    let program = &programs.get(name).unwrap();
    let mut weights = HashMap::new();

    for child in &program.children {
        let weight = total_weight(programs, child)?;
        let entry = weights.entry(weight).or_insert((child, 0));
        entry.0 = child;
        entry.1 += 1;
    }

    match weights.len() {
        0 => Ok(program.weight),
        1 => Ok(program.weight + weights.iter().next().unwrap().0 * program.children.len() as u32),
        2 => {
            let wrong_weight = *weights.iter().find(|(_, entry)| entry.1 == 1).unwrap().0;
            let wrong_name = weights.get(&wrong_weight).unwrap().0;
            let ok_weight = *weights
                .iter()
                .find(|(weight, _)| **weight != wrong_weight)
                .unwrap()
                .0;
            let diff = ok_weight as i32 - wrong_weight as i32;
            Err((programs.get(wrong_name).unwrap().weight as i32 + diff) as u32)
        }
        _ => Err(0),
    }
}

fn insert_or_remove_if_present(set: &mut HashSet<String>, name: &String) {
    if set.contains(name) {
        set.remove(name);
    } else {
        set.insert(name.clone());
    }
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let programs: HashMap<String, Program> = reader
        .lines()
        .map(|line| {
            let p: Program = line.unwrap().trim().parse().unwrap();
            (p.name.clone(), p)
        })
        .collect();

    let mut set: HashSet<String> = HashSet::new();
    programs.iter().for_each(|(name, program)| {
        insert_or_remove_if_present(&mut set, &name);
        program
            .children
            .iter()
            .for_each(|child| insert_or_remove_if_present(&mut set, &child));
    });

    assert_eq!(set.len(), 1);

    let bottom = set.iter().next().unwrap();

    println!("part 1: {}", bottom);
    println!("part 2: {}", total_weight(&programs, bottom).err().unwrap());
}
