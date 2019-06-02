use std::io;

fn knot_hash(lengths: &Vec<usize>, rounds: usize) -> Vec<u32> {
    let mut list: Vec<u32> = (0..256).collect();
    let mut pos = 0;
    let mut skip_size = 0;

    for _i in 0..rounds {
        for length in lengths {
            for i in 0..(length / 2) {
                let start = (pos + i) % list.len();
                let end = (pos + length - i - 1) % list.len();
                let tmp = list[start];
                list[start] = list[end];
                list[end] = tmp;
            }

            pos = (pos + length + skip_size) % list.len();
            skip_size += 1;
        }
    }

    list
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let lengths: Vec<usize> = input.split(',').map(|val| val.parse().unwrap()).collect();
    let list = knot_hash(&lengths, 1);
    println!("part 1: {}", list[0] * list[1]);

    let mut lengths: Vec<usize> = input.as_bytes().iter().map(|&b| b as usize).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let list = knot_hash(&lengths, 64);
    let dense_hash: Vec<u32> = (0..16)
        .map(|n| list.iter().skip(n * 16).take(16).fold(0, |acc, b| acc ^ b))
        .collect();
    print!("part 2: ");
    dense_hash.iter().for_each(|x| print!("{:02x}", x));
    println!();
}
