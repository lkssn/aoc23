fn main() {
    let input = std::fs::read_to_string("day12/data/input.txt").expect("failed to read file");
    let mut document = Document::parse(&input);

    let sum = document.arrangements_sum();
    println!("sum: {sum}");

    document.unfold();
    let sum_unfolded = document.arrangements_sum();
    println!("sum_unfolded: {sum_unfolded}");
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<i64>
}

impl Record {
    fn parse(s: &str) -> Record {
        let mut springs = Vec::new();
        let mut groups = Vec::new();
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

    fn arrangements(&self) -> i64 {
        let mut counter = 0;
        let mut sequences = vec![(self.springs.clone(), 1)];

        for g in self.groups.iter().take(self.groups.len() - 1) {
            let g = *g;

            let mut new_sequences = vec![];
            for sequence in &sequences {
                // Check: sequence == (O x z1, D x g, O, R)
                let mut damaged: i64 = 0;
                while damaged < (sequence.0.len() as i64) && sequence.0[damaged as usize] != Spring::Damaged {
                    damaged += 1;
                }

                let end = damaged.min(sequence.0.len() as i64 - 1 - g);
                let search_space = 0..=end;
                for start in search_space {
                    // check if g matches at start: {start, ..., start + g} = {D, ... , D, O}
                    let mut is_group_match = true;
                    for spring in &sequence.0[(start as usize)..((start + g) as usize)] {
                        is_group_match = is_group_match && [Spring::Damaged, Spring::Unknown].contains(spring);
                    }
                    is_group_match = is_group_match && [Spring::Operational, Spring::Unknown].contains(&sequence.0[(start+g) as usize]);

                    if is_group_match {
                        let new_sequence = (Vec::from(&sequence.0[((start+g+1) as usize)..]), sequence.1);
                        new_sequences.push(new_sequence);
                    }
                }
            }
            sequences.clear();
            sequences.append(&mut new_sequences);

            // most important part:
            // deduplicate the sequences set, so it does not grow expontentially
            // also take care to remember the amount of "points" each sequence carries
            sequences.sort();

            for i in 0..sequences.len() {
                let mut sum = sequences[i].1;
                for j in (i+1)..sequences.len() {
                    if sequences[i].0.eq(&sequences[j].0) {
                        sum += sequences[j].1;
                    } else {
                        break;
                    }
                }
                sequences[i].1 = sum;
            }

            sequences.dedup_by(|a, b| a.0.eq(&b.0));
        }

        // check the last group
        let g = *self.groups.last().unwrap();

        for sequence in sequences {
            // Check: sequence == (O x z1, D x g, O x z2)
            let mut damaged: i64 = 0;
            while damaged < (sequence.0.len() as i64) && sequence.0[damaged as usize] != Spring::Damaged {
                damaged += 1;
            }

            let end = damaged.min(sequence.0.len() as i64 - g);
            let search_space = 0..=end;
            for start in search_space {
                // check if g matches at start: {start, ..., start + g} = {D, ... , D} and O after that
                let mut is_group_match = true;
                for spring in &sequence.0[(start as usize)..((start + g) as usize)] {
                    is_group_match = is_group_match && [Spring::Damaged, Spring::Unknown].contains(spring);
                }

                for spring in &sequence.0[((start + g) as usize)..] {
                    is_group_match = is_group_match && [Spring::Operational, Spring::Unknown].contains(spring);
                }

                if is_group_match {
                    counter += sequence.1;
                }
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
        for record in &mut self.records {
            record.unfold();
        }
    }
}
