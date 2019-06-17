use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

type Image = Vec<Vec<u8>>;

fn pattern2image(pattern: &str) -> Image {
    pattern
        .split('/')
        .map(|row| row.as_bytes().to_vec())
        .collect()
}

fn resize_image(image: &mut Image, new_size: usize) {
    image.resize(new_size, vec![]);
    for row in image {
        row.resize(new_size, b'.');
    }
}

fn flip_image(image: &mut Image) {
    for row in image {
        let mut low = 0;
        let mut high = row.len() - 1;

        while low < high {
            let tmp = row[low];
            row[low] = row[high];
            row[high] = tmp;

            low += 1;
            high -= 1;
        }
    }
}

fn rotate_image(image: &mut Image) {
    let n = image.len();

    for i in 0..(n / 2) {
        for j in i..(n - i - 1) {
            let tmp = image[i][j];
            image[i][j] = image[n - j - 1][i];
            image[n - j - 1][i] = image[n - i - 1][n - j - 1];
            image[n - i - 1][n - j - 1] = image[j][n - i - 1];
            image[j][n - i - 1] = tmp;
        }
    }
}

fn find_enhancement<'a>(square: &Image, rules: &'a HashMap<Image, Image>) -> Option<&'a Image> {
    let mut square = square.clone();

    for _ in 0..4 {
        if rules.contains_key(&square) {
            return Some(rules.get(&square).unwrap());
        }

        let mut flip = square.clone();
        flip_image(&mut flip);
        if rules.contains_key(&flip) {
            return Some(rules.get(&flip).unwrap());
        }

        rotate_image(&mut square);
    }

    None
}

fn write_enhancement(image: &mut Image, enhancement: &Image, i: usize, j: usize) {
    for di in 0..enhancement.len() {
        for dj in 0..enhancement.len() {
            image[i + di][j + dj] = enhancement[di][dj];
        }
    }
}

fn iteration(image: &mut Image, rules: &HashMap<Image, Image>) {
    let size = image.len();
    let square_size = if size % 2 == 0 { 2 } else { 3 };
    resize_image(image, size + size / square_size);

    for i in (0..(size / square_size)).rev() {
        for j in (0..(size / square_size)).rev() {
            let si = i * square_size;
            let sj = j * square_size;
            let square = &image[si..(si + square_size)]
                .iter()
                .map(|row| row[sj..(sj + square_size)].to_vec())
                .collect();

            let enhancement = find_enhancement(square, &rules).expect("no enhancement found!");
            write_enhancement(
                image,
                &enhancement,
                i * (square_size + 1),
                j * (square_size + 1),
            );
        }
    }
}

fn pixels_on(image: &Image) -> usize {
    image.iter().flatten().filter(|&&x| x == b'#').count()
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let rules: HashMap<Image, Image> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let rule: Vec<&str> = line.split(" => ").collect();
            (pattern2image(rule[0]), pattern2image(rule[1]))
        })
        .collect();

    let mut image = vec![
        vec![b'.', b'#', b'.'],
        vec![b'.', b'.', b'#'],
        vec![b'#', b'#', b'#'],
    ];

    for _ in 0..5 {
        iteration(&mut image, &rules);
    }
    println!("part 1: {}", pixels_on(&image));

    for _ in 5..18 {
        iteration(&mut image, &rules);
    }
    println!("part 2: {}", pixels_on(&image));
}
