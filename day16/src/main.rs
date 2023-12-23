use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("day16/data/input.txt").expect("failed to read file");
    let mut map = Map{w:0, h:0, tiles: vec![]};

    for line in input.lines() {
        map.h += 1;
        for c in line.chars() {
            let space = match c {
                '.' => Space::Empty,
                '/' => Space::LeanRight,
                '\\' => Space::LeanLeft,
                '|' => Space::Vertical,
                '-' => Space::Horizontal,
                _ => panic!("invalid space")
            };
            let tile = Tile::new(space);
            map.tiles.push(tile);
        }
    }
    map.w = map.tiles.len() as i32 / map.h;

    let w = map.w;
    let h = map.h;
    let first_beam = Beam {x: 0, y: 0, d: Direction::East};
    let first_energized = map.energized(first_beam);
    println!("first_energized: {}", first_energized);

    let mut max_energized = 0;
    let starts_top = zip(zip(0..w, [0].into_iter().cycle()), [Direction::South].into_iter().cycle());
    let starts_bottom = zip(zip(0..w, [h-1].into_iter().cycle()), [Direction::North].into_iter().cycle());
    let starts_left = zip(zip([0].into_iter().cycle(), 0..h), [Direction::East].into_iter().cycle());
    let starts_right = zip(zip([w-1].into_iter().cycle(), 0..h), [Direction::West].into_iter().cycle());
    
    for ((x, y), d) in starts_top.chain(starts_bottom).chain(starts_left).chain(starts_right) {
        let beam = Beam{x,y,d};
        let energized = map.energized(beam);
        max_energized = max_energized.max(energized);
    }

    println!("max_energized: {}", max_energized);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

struct Beam {
    x: i32,
    y: i32,
    d: Direction
}

#[derive(Debug, Clone, Copy)]
enum Space {
    Empty,
    LeanRight,
    LeanLeft,
    Vertical,
    Horizontal
}

struct Tile {
    space: Space,
    north: bool,
    east: bool,
    south: bool,
    west: bool
}

impl Tile {
    fn new(space: Space) -> Tile {
        Tile{space, north: false, east: false, south: false, west: false}
    }

    fn energized(&self) -> bool {
        self.north || self.east || self.south || self.west
    }

    fn reset(&mut self) {
        self.north = false;
        self.east = false;
        self.south = false;
        self.west = false;
    }
}

struct Map {
    w: i32,
    h: i32,
    tiles: Vec<Tile>
}

impl Map {
    fn get_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        let index = y * self.w + x;
        &mut self.tiles[index as usize]
    }

    fn reset(&mut self) {
        for tile in &mut self.tiles {
            tile.reset();
        }
    }

    fn energized(&mut self, beam: Beam) -> i32 {
        self.reset();

        let mut beams = vec![beam];
        let w = self.w;
        let h = self.h;

        while !beams.is_empty() {
            let beam = beams.pop().unwrap();

            // skip out of bounds beams
            if !(0..w).contains(&beam.x) {
                continue;
            }
            if !(0..h).contains(&beam.y) {
                continue;
            }

            let tile = self.get_mut(beam.x, beam.y);
            let repetition = match beam.d {
                Direction::North => tile.north,
                Direction::East  => tile.east,
                Direction::South => tile.south,
                Direction::West  => tile.west,
            };

            // skip loops
            if repetition {
                continue;
            }

            // remember beams
            match beam.d {
                Direction::North => tile.north = true,
                Direction::East  => tile.east = true,
                Direction::South => tile.south = true,
                Direction::West  => tile.west = true,
            };

            let dirs = match (tile.space, beam.d) {
                (Space::Empty, _) => vec![beam.d],

                (Space::LeanRight, Direction::North) => vec![Direction::East],
                (Space::LeanRight, Direction::East)  => vec![Direction::North],
                (Space::LeanRight, Direction::South) => vec![Direction::West],
                (Space::LeanRight, Direction::West)  => vec![Direction::South],

                (Space::LeanLeft, Direction::North) => vec![Direction::West],
                (Space::LeanLeft, Direction::East)  => vec![Direction::South],
                (Space::LeanLeft, Direction::South) => vec![Direction::East],
                (Space::LeanLeft, Direction::West)  => vec![Direction::North],

                (Space::Vertical, Direction::East | Direction::West)  => vec![Direction::North, Direction::South],
                (Space::Vertical, _) => vec![beam.d],

                (Space::Horizontal, Direction::North | Direction::South) => vec![Direction::East, Direction::West],
                (Space::Horizontal, _)  => vec![beam.d],
            };

            for d in dirs {
                let (x, y) = (beam.x, beam.y);
                let (x, y) = match d {
                    Direction::North => (x, y-1),
                    Direction::East  => (x+1, y),
                    Direction::South => (x, y+1),
                    Direction::West  => (x-1, y),
                };

                let new_beam = Beam{x, y, d};
                beams.push(new_beam);
            }
        }

        let mut energized = 0;
        for tile in &self.tiles {
            if tile.energized() {
                energized += 1;
            }
        }

        energized
    }
}
