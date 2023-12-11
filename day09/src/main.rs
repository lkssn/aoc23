use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("day09/data/input.txt").expect("failed to read file");
    let (sum_prev, sum_next): (i32, i32) = input.lines().map(|line| History::parse(line).predict()).fold((0,0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));
    println!("sum_prev: {sum_prev}");
    println!("sum_next: {sum_next}");
}

#[derive(Debug, Clone)]
struct History {
    values: Vec<i32>
}

impl History {
    fn parse(s: &str) -> History {
        History {
            values: s.split_whitespace().map(|x| x.parse().unwrap()).collect()
        }
    }

    fn differences(&self) -> History {
        History {
            values: zip(self.values.iter(), self.values.iter().skip(1)).map(|(x, y)| y - x).collect()
        }
    }

    fn is_zero(&self) -> bool {
        self.values.iter().all(|x| *x == 0)
    }

    fn predict(&self) -> (i32, i32) {
        let mut histories = vec![self.clone()];

        while !histories.last().unwrap().is_zero() {
            histories.push(histories.last().unwrap().differences());
        }

        let mut prev = 0;
        let mut next = 0;
        for history in histories.iter().rev() {
            prev = history.values.first().unwrap() - prev;
            next = history.values.last().unwrap() + next;
        }

        (prev, next)
    }
}