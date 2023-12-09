use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("day06/data/input.txt").expect("failed to read file");
    let mut lines = input.lines();
    let times = lines.next().expect("failed to read times").split(':').skip(1).next().expect("failed to split times");
    let distances = lines.next().expect("failed to read distances").split(':').skip(1).next().expect("failed to split distances");
    let races = Race::parse_multiple(times, distances);
    let race = Race::parse_single(times, distances);
    println!("multiple races: {races:?}");
    println!("single race: {race:?}");

    let product = races.iter().map(Race::exceeding_button_times).product::<i64>();
    let combinations = race.exceeding_button_times();

    println!("product: {product}");
    println!("combinations: {combinations}");
}

#[derive(Debug)]
struct Race {
    duration: i64,
    record: i64
}

impl Race {
    fn parse_multiple(times: &str, distances: &str) -> Vec<Race> {
        zip(times.split_whitespace(), distances.split_whitespace())
        .map(|(time, distance)|
            Race {
                duration: time.parse().expect("parse time"),
                record: distance.parse().expect("parse distance")
            }
        ).collect()
    }

    fn parse_single(times: &str, distances: &str) -> Race {
        let (time, distance) = zip(times.split_whitespace(), distances.split_whitespace())
        .fold((String::new(), String::new()), |(mut acc_t, mut acc_d), (t, d)| {
                acc_t.push_str(t);
                acc_d.push_str(d);
                (acc_t, acc_d)
            }
        );

        Race {
            duration: time.parse().expect("parse time"),
            record: distance.parse().expect("parse distance")
        }
    }

    // can propably improve this with some sort of binary search
    // 0 <= counter <= duration + 1
    // but it is fast enough anyways, so there is no need to do that
    fn exceeding_button_times(&self) -> i64 {
        let mut counter = 0;
        for button_time in 0..=(self.duration+1) {
            counter += if self.is_exceeding_button_time(button_time) {1} else {0};
        }
        counter
    }

    fn is_exceeding_button_time(&self, button_time: i64) -> bool {
        if button_time < 0 || button_time > self.duration {
            return false;
        }

        let speed = button_time;
        let remaining_time = self.duration - button_time;
        let distance = speed * remaining_time;

        distance > self.record
    }
}
