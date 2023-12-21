type BigInt = i64;

fn main() {
    let input = std::fs::read_to_string("day15/data/input.txt").expect("failed to read file");
    let mut boxes = vec![Box::new(); 256];
    let mut sum = 0;

    for step in input.split(&[',']) {
        let h = hash(step);
        sum += h;
        println!("- '{}' becomes {}", step, h);

        if step.contains('=') {
            let mut parts = step.split('=');
            let label = parts.next().unwrap();
            let focal_length: BigInt = parts.next().unwrap().parse().unwrap();
            let box_index = hash(label);
            let lens = Lens{label, focal_length};
            let lenses = &mut boxes[box_index as usize].lenses;

            if lenses.contains(&lens) {
                for it in lenses.iter_mut().filter(|x| x.label == lens.label) {
                    it.focal_length = focal_length;
                }
            } else {
                lenses.push(lens);
            }
        } else if step.contains('-') {
            let label = &step[0..(step.len()-1)];
            let box_index = hash(label);
            let lenses = &mut boxes[box_index as usize].lenses;
            lenses.retain_mut(|x| x.label != label);
        } else {
            panic!("invalid step: {}", step);
        }
    }

    println!("sum: {sum}");

    let mut sum_power = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, l) in b.lenses.iter().enumerate() {
            let mut power = i as BigInt + 1;
            power *= j as BigInt + 1;
            power *= l.focal_length;
            sum_power += power;
        }
    }

    println!("sum_power: {sum_power}");
}

fn hash(s: &str) -> BigInt {
    let mut h = 0;

    for c in s.chars() {
        let v = c as BigInt;
        h += v;
        h *= 17;
        h %= 256;
    }

    h
}

#[derive(Debug,Clone)]
struct Box<'a> {
    lenses: Vec<Lens<'a>>
}

impl Box<'_> {
    fn new<'a>() -> Box<'a> {
        Box {
            lenses: vec![]
        }
    }
}

#[derive(Debug,Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: BigInt
}

impl PartialEq for Lens<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}