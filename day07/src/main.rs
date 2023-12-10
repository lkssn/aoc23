use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = std::fs::read_to_string("day07/data/input.txt").expect("failed to read file");
    let mut hands = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let cards_str = parts.next().expect("failed to read hand cards");
        let bid_str = parts.next().expect("failed to read hand bid");
        let hand = Hand::parse(cards_str, bid_str);
        hands.push(hand);
    }

    hands.sort_by(Hand::cmp);
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = (i + 1) as i32;
        winnings += hand.bid * rank;
    }
    println!("winnings: {winnings}");

    hands.sort_by(Hand::cmp2);
    let mut winnings2 = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = (i + 1) as i32;
        winnings2 += hand.bid * rank;
    }
    println!("winnings2: {winnings2}");
}

#[derive(Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    fn value(&self) -> u8 {
        match &self {
            HandType::FiveKind  => 7,
            HandType::FourKind  => 6,
            HandType::FullHouse => 5,
            HandType::ThreeKind => 4,
            HandType::TwoPair   => 3,
            HandType::OnePair   => 2,
            HandType::HighCard  => 1
        }
    }

    fn cmp(&self, other: &HandType) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card;5],
    bid: i32
}

// idea for fast comparison: map every hand to a single number (maybe radix sort useful too?)
impl Hand {
    fn new() -> Hand {
        Hand {
            cards: [Card::A; 5],
            bid: 0
        }
    }

    fn parse(cards_str: &str, bid_str: &str) -> Hand {
        let mut hand = Hand::new();

        for (i, c) in cards_str.chars().enumerate() {
            hand.cards[i] = Card::parse(c);
        }

        hand.bid = bid_str.parse().expect("failed to parse bid");
        hand
    }

    fn hand_type(&self) -> HandType {
        let mut kinds = HashMap::new();

        for card in &self.cards {
            *kinds.entry(card).or_insert(0) += 1;
        }

        let mut cardinalities = kinds.values().map(|x| *x).collect::<Vec::<i32>>();
        cardinalities.sort();
        let card_str = cardinalities.iter().fold(String::new(), |mut acc, c| {acc.push(char::from_digit(*c as u32, 10).unwrap()); acc});

        match card_str.as_str() {
            "5" => HandType::FiveKind,
            "14" => HandType::FourKind,
            "23" => HandType::FullHouse,
            "113" => HandType::ThreeKind,
            "122" => HandType::TwoPair,
            "1112" => HandType::OnePair,
            "11111" => HandType::HighCard,
            _ => panic!("failed to parse hand type")
        }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        let cmp_type = self.hand_type().cmp(&other.hand_type());
        if !cmp_type.is_eq() {
            return cmp_type;
        }

        for i in 0..5 {
            let cmp_card = self.cards[i].cmp(&other.cards[i]);
            if !cmp_card.is_eq() {
                return cmp_card;
            }
        }

        Ordering::Equal
    }

    fn hand_type2(&self) -> HandType {
        let mut kinds = HashMap::new();

        for card in &self.cards {
            *kinds.entry(card).or_insert(0) += 1;
        }

        let mut cardinalities = kinds.values().map(|x| *x).collect::<Vec::<i32>>();
        cardinalities.sort();
        let card_str = cardinalities.iter().fold(String::new(), |mut acc, c| {acc.push(char::from_digit(*c as u32, 10).unwrap()); acc});
        let jokers = *kinds.get(&Card::J).unwrap_or(&0);

        match card_str.as_str() {
            "5" => HandType::FiveKind,
            "14" => {
                match jokers {
                    1 => HandType::FiveKind,
                    4 => HandType::FiveKind,
                    _ => HandType::FourKind
                }
            },
            "23" => {
                match jokers {
                    2 => HandType::FiveKind,
                    3 => HandType::FiveKind,
                    _ => HandType::FullHouse
                }
            }
            "113" => {
                match jokers {
                    1 => HandType::FourKind,
                    3 => HandType::FourKind,
                    _ => HandType::ThreeKind
                }
            },
            "122" => {
                match jokers {
                    1 => HandType::FullHouse,
                    2 => HandType::FourKind,
                    _ => HandType::TwoPair,
                }
            },
            "1112" => {
                match jokers {
                    1 => HandType::ThreeKind,
                    2 => HandType::ThreeKind,
                    _ => HandType::OnePair,
                }
            },
            "11111" => {
                match jokers {
                    1 => HandType::OnePair,
                    _ => HandType::HighCard
                }
            },
            _ => panic!("failed to parse hand type")
        }
    }

    fn cmp2(&self, other: &Hand) -> Ordering {
        let cmp_type = self.hand_type2().cmp(&other.hand_type2());
        if !cmp_type.is_eq() {
            return cmp_type;
        }

        for i in 0..5 {
            let cmp_card = self.cards[i].cmp2(&other.cards[i]);
            if !cmp_card.is_eq() {
                return cmp_card;
            }
        }

        Ordering::Equal
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Card {
    fn parse(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("failed card parse")
        }
    }

    fn value(&self) -> u8 {
        match &self {
            Card::A     => 14,
            Card::K     => 13,
            Card::Q     => 12,
            Card::J     => 11,
            Card::T     => 10,
            Card::Nine  => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six   => 6,
            Card::Five  => 5,
            Card::Four  => 4,
            Card::Three => 3,
            Card::Two   => 2
        }
    }

    fn value2(&self) -> u8 {
        match &self {
            Card::A     => 14,
            Card::K     => 13,
            Card::Q     => 12,
            Card::J     => 0,
            Card::T     => 10,
            Card::Nine  => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six   => 6,
            Card::Five  => 5,
            Card::Four  => 4,
            Card::Three => 3,
            Card::Two   => 2
        }
    }

    fn cmp(&self, other: &Card) -> Ordering {
        self.value().cmp(&other.value())
    }

    fn cmp2(&self, other: &Card) -> Ordering {
        self.value2().cmp(&other.value2())
    }
}
