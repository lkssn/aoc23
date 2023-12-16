fn main() {
    let input = std::fs::read_to_string("day14/data/input.txt").expect("failed to read file");

    let mut platform = Platform{rocks: vec![]};
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let rock = match c {
                'O' => Rock::Round,
                '#' => Rock::Cube,
                '.' => Rock::Empty,
                _ => panic!("no rock")
            };
            row.push(rock);
        }
        platform.rocks.push(row);
    }

    let w = platform.rocks[0].len();
    let h = platform.rocks.len();
    let mut sum_simple = 0;
    let cycles = 1000000000;
    let mut previous_platforms = vec![];
    let mut period = 0;
    let mut repeat_index = 0;

    // one cycle: north->west->south->east
    for cycle in 0..cycles {
        previous_platforms.push(platform.clone());

        // north: y=0,...,h-1
        let mut limits_north = vec![-1 as i64; w];
        for y in 0..h {
            for x in 0..w {
                match platform.rocks[y][x] {
                    Rock::Round => {
                        let pos = limits_north[x] + 1;
                        limits_north[x] = pos;

                        platform.rocks[y][x] = Rock::Empty;
                        platform.rocks[pos as usize][x] = Rock::Round;

                        if cycle == 0 {
                            sum_simple += h as i64 - pos;
                        }
                    },
                    Rock::Cube => {
                        limits_north[x] = y as i64;
                    },
                    Rock::Empty => {
                        // don't care
                    }
                }
            }
        }

        // west: x=0,...,w-1
        let mut limits_west = vec![-1 as i64; h];
        for x in 0..w {
            for y in 0..h {
                match platform.rocks[y][x] {
                    Rock::Round => {
                        let pos = limits_west[y] + 1;
                        limits_west[y] = pos;

                        platform.rocks[y][x] = Rock::Empty;
                        platform.rocks[y][pos as usize] = Rock::Round;
                    },
                    Rock::Cube => {
                        limits_west[y] = x as i64;
                    },
                    Rock::Empty => {
                        // don't care
                    }
                }
            }
        }

        // south: y=h-1,...,0
        let mut limits_south = vec![h as i64; w];
        for y in (0..h).rev() {
            for x in 0..w {
                match platform.rocks[y][x] {
                    Rock::Round => {
                        let pos = limits_south[x] - 1;
                        limits_south[x] = pos;

                        platform.rocks[y][x] = Rock::Empty;
                        platform.rocks[pos as usize][x] = Rock::Round;
                    },
                    Rock::Cube => {
                        limits_south[x] = y as i64;
                    },
                    Rock::Empty => {
                        // don't care
                    }
                }
            }
        }

        // east: x=w-1,...,0
        let mut limits_east = vec![w as i64; h];
        for x in (0..w).rev() {
            for y in 0..h {
                match platform.rocks[y][x] {
                    Rock::Round => {
                        let pos = limits_east[y] - 1;
                        limits_east[y] = pos;

                        platform.rocks[y][x] = Rock::Empty;
                        platform.rocks[y][pos as usize] = Rock::Round;
                    },
                    Rock::Cube => {
                        limits_east[y] = x as i64;
                    },
                    Rock::Empty => {
                        // don't care
                    }
                }
            }
        }

        if previous_platforms.contains(&platform) {
            let completed_cycles = cycle + 1;

            for (i, prev) in previous_platforms.iter().enumerate() {
                if prev.eq(&platform) {
                    println!("Repetition at indices {}<->{}", i, completed_cycles);
                    repeat_index = i;
                    period = completed_cycles - i;
                }
            }
            break;
        }
    }

    // calculate the index after every cycle is completed
    let remaining = cycles - repeat_index;
    let remaining_rest = remaining % period;
    let index = repeat_index + remaining_rest;
    let cycle_platform = &previous_platforms[index];

    let mut sum_cycle = 0;
    for y in 0..h {
        for x in 0..w {
            if cycle_platform.rocks[y][x] == Rock::Round {
                sum_cycle += h as i64 - y as i64;
            }
        }
    }

    println!("sum_simple: {}", sum_simple);
    println!("sum_cycle: {}", sum_cycle);
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Rock {
    Round,
    Cube,
    Empty
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Platform {
    rocks: Vec<Vec<Rock>>
}

impl Platform {
    fn print(&self) {
        let w = self.rocks[0].len();
        let h = self.rocks.len();

        for y in 0..h {
            for x in 0..w {
                let c = match self.rocks[y][x] {
                    Rock::Round => 'O',
                    Rock::Cube  => '#',
                    Rock::Empty => '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }
}