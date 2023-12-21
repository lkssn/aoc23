type BigInt = i64;

fn main() {
    let input = std::fs::read_to_string("day15/data/input.txt").expect("failed to read file");
    let mut sum = 0;

    for step in input.split(&[',']) {
        let h = hash(step);
        sum += h;
        println!("- '{}' becomes {}", step, h);
    }

    println!("sum: {sum}");
}

fn hash(s: &str) -> BigInt {
    let mut h = 0;

    for c in s.chars() {
        if c.is_whitespace() {
            continue;
        }

        let v = c as BigInt;
        h += v;
        h *= 17;
        h %= 256;
    }

    h
}