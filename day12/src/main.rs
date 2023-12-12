fn main() {
    let input = std::fs::read_to_string("day12/data/example.txt").expect("failed to read file");
    let document = Document::parse(&input);

    println!("document: {document:#?}")
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
}
