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

    let beam = Beam {x: 0, y: 0, d: Direction::East};
    let mut beams = vec![beam];
    let w = map.w;
    let h = map.h;

    while !beams.is_empty() {
        let beam = beams.pop().unwrap();

        // skip out of bounds beams
        if !(0..w).contains(&beam.x) {
            continue;
        }
        if !(0..h).contains(&beam.y) {
            continue;
        }

        let tile = map.get_mut(beam.x, beam.y);
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
    for tile in &map.tiles {
        if tile.energized() {
            energized += 1;
        }
    }

    println!("energized: {}", energized);
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
}

struct Map {
    w: i32,
    h: i32,
    tiles: Vec<Tile>
}

impl Map {
    fn get(&self, x: i32, y: i32) -> &Tile {
        let index = y * self.w + x;
        &self.tiles[index as usize]
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        let index = y * self.w + x;
        &mut self.tiles[index as usize]
    }
}