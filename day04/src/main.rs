use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};

fn main() {
    let file = File::open("day04/data/input.txt").expect("file failed");
    let reader = BufReader::new(file);
    let mut sum_scores = 0;
    let mut sum_totals = 0;

    let mut copies = VecDeque::<i64>::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let card  = Card::parse(&line);
            let matches = card.matches();
            let score = calc_points(matches);
            let total = 1 + copies.pop_front().unwrap_or(0); 

            // updates for current card
            println!("matches: {matches}");
            println!("score: {score}");
            println!("total: {total}");

            // updates for future cards
            if matches as usize > copies.len() {
                copies.resize_with(matches as usize, || 0);
            }

            for (offset, copy_count) in std::iter::repeat(total).take(matches as usize).enumerate() {
                copies[offset] += copy_count;
            }
            sum_scores += score;
            sum_totals += total;
        }
    }

    println!("sum_scores: {sum_scores}");
    println!("sum_totals: {sum_totals}");
}

#[derive(Debug)]
struct Card {
    winners: Vec<i64>,
    owned: Vec<i64>,
}


impl Card {
    fn new() -> Card {
        Card { winners: vec![], owned: vec![]}
    }

    fn parse(s: &str) -> Card {
        let mut parts =  s.split(':').skip(1).next().expect("failed to parse card").split('|');
        let first = parts.next().expect("failed to parse card: winners");
        let second = parts.next().expect("failed to parse card: owned");
        let mut card = Card::new();

        for w in first.split_ascii_whitespace() {
            let num = w.parse().expect("failed to parse card: winner number");
            card.winners.push(num);
        }

        for o in second.split_ascii_whitespace() {
            let num = o.parse().expect("failed to parse card: owner number");
            card.owned.push(num);
        }

        card.winners.sort();
        card.owned.sort();

        return card;
    }

    fn matches(&self) -> i64 {
        let mut k = 0;

        for w in &self.winners {
            if self.owned.binary_search(&w).is_ok() {
                k += 1;
            }
        }

        k
    }
}

fn calc_points(matches: i64) -> i64 {
    if matches >= 1 {
        1 << (matches-1)
    } else {
        0
    }
}
