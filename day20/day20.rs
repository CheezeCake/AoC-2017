use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl FromStr for Coordinates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '<' || p == '>')
            .split(',')
            .collect();

        Ok(Coordinates {
            x: coords[0].parse()?,
            y: coords[1].parse()?,
            z: coords[2].parse()?,
        })
    }
}

struct Particle {
    position: Coordinates,
    velocity: Coordinates,
    acceleration: Coordinates,
}

impl FromStr for Particle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .split(", ")
            .map(|coord| {
                let coord: Vec<&str> = coord.split('=').collect();
                coord[1]
            })
            .collect();

        Ok(Particle {
            position: coords[0].parse()?,
            velocity: coords[1].parse()?,
            acceleration: coords[2].parse()?,
        })
    }
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let mut particles: Vec<Particle> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // let mut min = 0;
    // for (i, p) in particles.iter().enumerate() {
    //     let si = p.acceleration.x.abs() + p.acceleration.y.abs() + p.acceleration.z.abs();
    //     let sm = particles[min].acceleration.x.abs()
    //         + particles[min].acceleration.y.abs()
    //         + particles[min].acceleration.z.abs();
    //     if si < sm {
    //         min = i;
    //     }
    // }

    // println!("part 1: {}", min);

    let mut collided = HashSet::new();
    for _ in 0..1000 {
        let mut positions: HashMap<Coordinates, Vec<usize>> = HashMap::new();
        for (i, p) in particles.iter_mut().enumerate() {
            p.velocity += p.acceleration;
            p.position += p.velocity;

            let entry = positions.entry(p.position).or_insert(vec![]);
            entry.push(i);
        }
        for (_, indices) in positions {
            if indices.len() > 1 {
                for index in indices {
                    collided.insert(index);
                }
            }
        }
    }

    println!(
        "part 1: {}",
        particles
            .iter()
            .enumerate()
            .map(|(i, p)| {
                (
                    i,
                    p.position.x.abs() + p.position.y.abs() + p.position.z.abs(),
                )
            })
            .min_by_key(|x| x.1)
            .unwrap()
            .0
    );

    println!("part 2: {}", particles.len() - collided.len());
}
