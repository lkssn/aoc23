fn main() {
    let record_str = std::fs::read_to_string("day02/data/input.txt").expect("failed to read file");
    let expectation = CubeSet{red: 12, green: 13, blue: 14};

    let record = read_record(&record_str, expectation).expect("failed to read record");
    let sum = record.calc_sum();
    println!("sum: {sum}");

    let sum_power_max = record.sum_power_max();
    println!("sum_power_max: {sum_power_max}");
}


#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    sets: Vec<CubeSet>
}

struct Record {
    expectation: CubeSet,
    games: Vec<Game>
}

impl Game {
    fn check_game(&self, expectation: &CubeSet) -> bool {
        self.sets.iter().all(|set| set.subset(expectation))
    }

    fn calc_max(&self) -> CubeSet {
        let mut max = CubeSet{red: 0, green: 0, blue: 0};

        for set in &self.sets {
            max.red = max.red.max(set.red);
            max.green = max.green.max(set.green);
            max.blue = max.blue.max(set.blue);
        }

        return max;
    }
}

impl Record {
    fn calc_sum(&self) -> u32 {
        let mut sum = 0;

        for game in &self.games {
            if game.check_game(&self.expectation) {
                sum += game.id;
            }
        }

        return sum;
    }

    fn sum_power_max(&self) -> u32 {
        let mut sum = 0;

        for game in &self.games {
            sum +=  game.calc_max().power();
        }

        return sum;
    }
}

impl CubeSet {
    fn subset(&self, other: &CubeSet) -> bool {
        return self.red <= other.red && self.green <= other.green && self.blue <= other.blue;
    }

    fn add(&self, other: &CubeSet) -> CubeSet {
        CubeSet{
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }

    fn power(&self) -> u32 {
       self.red * self.green * self.blue
    }
}

fn read_color(color_str: &str) -> Option<CubeSet> {
    let mut set = CubeSet{red: 0, green: 0, blue: 0};

    let parts = color_str.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 2 {
        println!("read_color: error parts len");
        return None;
    }

    let first = parts[0].trim().parse::<u32>();
    if first.is_err() {
        println!("read_color: error parsing first string");
        return None;
    }
    let count = first.unwrap();

    let second = parts[1].trim();
    match second {
        "red" => {set.red = count;},
        "green" => {set.green = count;},
        "blue" => {set.blue = count;},
        _ => {
            println!("read_color: error parsing second string");
            return None;
        }
    }

    return Some(set);
}

fn read_set(config_str: &str) -> Option<CubeSet> {
    let mut acc = CubeSet{red: 0, green: 0, blue: 0};

    for color_str in config_str.split(',') {
        if let Some(set) = read_color(color_str) {
            acc = acc.add(&set);
        } else {
            println!("read_set: read_set failed");
            return None;
        }     
    }

    Some(acc)
}

fn read_game(game_str: &str) -> Option<Game> {
    let parts = game_str.split(':').collect::<Vec<&str>>();
    if parts.len() != 2 {
        println!("read_game: parts != 2");
        return None;
    }

    let mut game = Game{id:0, sets: vec![]};
    let number_str = parts[0].split_whitespace().skip(1).collect::<Vec<&str>>()[0];
    if let Ok(number) = number_str.parse::<u32>() {
        game.id = number;
    } else {
        println!("read_game: number failed");
        return None;
    }

    for set_str in parts[1].split(';') {
        if let Some(set) = read_set(set_str) {
            game.sets.push(set);
        } else {
            println!("read_game: read_set failed");
            return None;
        }    
    }

    return Some(game);
}

fn read_record(record_str: &str, expectation: CubeSet) -> Option<Record> {
    let mut record = Record {expectation: expectation, games: vec![]};

    for game_str in record_str.lines() {
        if let Some(game) = read_game(game_str) {
            record.games.push(game);
        } else {
            println!("read_record: could not read game");
            return None;
        }
    }

    return Some(record);
}