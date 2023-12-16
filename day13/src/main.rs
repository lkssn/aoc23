use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("day13/data/input.txt").expect("failed to read file");

    let mut patterns = vec![];
    let mut pattern = vec![];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = vec![];
        } else {
            pattern.push(line);
        }
    }
    patterns.push(pattern);

    let mut sum = 0;
    let mut smudge_sum = 0;
    for pattern in &patterns {
        let w = pattern[0].len();
        let h = pattern.len();

        let column_reflections = zip(0..w, 1..w);
        let row_reflections = zip(0..h, 1..h);

        for (c0, c1) in column_reflections {
            let mut smudges = 0;
            for y in 0..h {
                for offset in 0.. {
                    if offset > c0 || offset + c1 > w - 1{
                        break;
                    }

                    let a = pattern[y].as_bytes()[c0 - offset];
                    let b = pattern[y].as_bytes()[c1 + offset];
                    if a != b {
                        smudges += 1;
                    }
                }
            }

            // found reflection
            if smudges == 0 {
                sum += c0 + 1;
                println!("column: {},{}", c0,c1);
            }

            if smudges == 1 {
                smudge_sum += c0 + 1;
                println!("column(smudged): {},{}", c0,c1);
            }
        }

        for (r0, r1) in row_reflections {
            let mut smudges = 0;
            for x in 0..w {
                for offset in 0.. {
                    if offset > r0 || offset + r1 > h - 1{
                        break;
                    }

                    let a = pattern[r0 - offset].as_bytes()[x];
                    let b = pattern[r1 + offset].as_bytes()[x];
                    if a != b {
                        smudges += 1;
                    }
                }
            }

            // found reflection
            if smudges == 0 {
                sum += (r0 + 1) * 100;
                println!("row: {},{}", r0, r1);
            }

            if smudges == 1 {
                smudge_sum += (r0 + 1) * 100;
                println!("row(smudged): {},{}", r0, r1);
            }
        }
    }

    println!("sum: {}", sum);
    println!("sum(smudged): {}", smudge_sum);
}
