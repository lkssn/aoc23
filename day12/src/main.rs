use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("day12/data/example.txt").expect("failed to read file");
    let mut document = Document::parse(&input);

    let sum = document.arrangements_sum();
    println!("sum: {sum}");

    document.unfold();
    let sum_unfolded = document.arrangements_sum();
    println!("sum_unfolded: {sum_unfolded}");
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

impl Spring {
    fn parse(c: char) -> Spring {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("failed to parse spring")
        }
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<i64>
}

impl Record {
    fn parse(s: &str) -> Record {
        let mut springs = vec![];
        let mut groups = vec![];
        let mut parts = s.split_whitespace();

        for spring_char in parts.next().unwrap().chars() {
            let spring = Spring::parse(spring_char);
            springs.push(spring);
        }

        for group_str in parts.next().unwrap().split(',') {
            let group = group_str.parse().unwrap();
            groups.push(group);
        }

        Record{springs, groups}
    }

    fn check_known(&self) -> bool {
        let mut group_counter = 0;
        let mut actual_groups = vec![];

        for spring in &self.springs {
            match spring {
                Spring::Damaged => {
                    group_counter += 1;
                },
                Spring::Operational => {
                    if group_counter > 0 {
                        actual_groups.push(group_counter);
                        group_counter = 0;
                    }
                },
                Spring::Unknown => panic!("check_known: found unknown spiral")
            }
        }

        if group_counter > 0 {
            actual_groups.push(group_counter);
        }

        if actual_groups.len() != self.groups.len() {
            return false;
        }

        zip(actual_groups.iter(), self.groups.iter()).all(|(x,y)| x == y)
    }

    fn arrangements(&self) -> i64 {
        let mut counter = 0;
        let mut known_record = Record{springs: self.springs.clone(), groups: self.groups.clone()};
        let unknowns = self.springs.iter().enumerate().filter(|(_, spring)| **spring == Spring::Unknown).map(|(i, _)| i).collect::<Vec::<usize>>();
        let u = unknowns.len();

        // Runtime is exponential with Omega(2^u): This is not feasible!
        // -> Have to solve the combinatorial problem on paper first.

        // encode the u picks with u bits
        // 1 = damaged, 0 = operational
        println!("arrangements bits: {u}");
        for damaged_pick in 0..(1 << u) {
            for i in 0..u {
                let damaged = ((damaged_pick >> i) & 1) == 1;
                known_record.springs[unknowns[i]] = if damaged {Spring::Damaged} else {Spring::Operational};
            }

            if known_record.check_known() {
                counter += 1;
            }
        }

        counter
    }

    fn unfold(&mut self) {
        let mut new_springs = self.springs.clone();
        let mut new_groups = self.groups.clone();

        for _ in 0..4 {
            new_springs.push(Spring::Unknown);
            new_springs.append(&mut self.springs.clone());
            new_groups.append(&mut self.groups.clone())
        }

        self.springs = new_springs;
        self.groups = new_groups;
    }
}

#[derive(Debug)]
struct Document {
    records: Vec<Record>
}

impl Document {
    fn parse(s: &str) -> Document {
        let mut records = vec![];

        for line in s.lines() {
            let record = Record::parse(line);
            records.push(record);
        }

        Document{records}
    }

    fn arrangements_sum(&self) -> i64 {
        self.records.iter().map(Record::arrangements).sum()
    }

    fn unfold(&mut self) {
        for record in self.records.iter_mut() {
            record.unfold();
        }
    }
}
