fn main() {
    let mut b: u64 = 57 * 100 + 100_000;
    let c: u64 = b + 17_000;
    let mut h: u64 = 0;

    loop {
        let mut f = 1;
        let mut d = 2;

        loop {
            let mut e = 2;

            loop {
                let de = d * e;
                if de > b {
                    break;
                }
                if d * e == b {
                    f = 0;
                    break;
                }

                e += 1;
            }
            if d == b || f == 0 {
                break;
            }

            d += 1;
        }

        if f == 0 {
            h += 1;
        }

        if b == c {
            break;
        }
        b += 17;
    }

    println!("part 2: {}", h);
}
