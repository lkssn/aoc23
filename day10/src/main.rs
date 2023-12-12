fn main() {
    let input = std::fs::read_to_string("day10/data/input.txt").expect("failed to read file");
    let mut sketch = Sketch::parse(input.as_str());
    let path = sketch.mark_loop();
    let distance = path.links.len() / 2;
    sketch.mark_small(&path);
    let enclosed = sketch.mark_big();

    sketch.print();
    println!("distance: {distance}");
    println!("enclosed: {enclosed}");
}

#[derive(Debug,PartialEq,Clone, Copy)]
enum Kind {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

#[derive(Debug,PartialEq,Clone,Copy)]
enum Topology {
    Boundary,
    Inside,
    Outside,
    Unknown
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    kind: Kind,
    topology: Topology
}

impl Tile {
    fn parse(c: char) -> Tile {
        Tile {
            kind: Kind::parse(c),
            topology: Topology::Unknown
        }
    }
}

#[derive(Debug,PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Kind {
    fn parse(c: char) -> Kind {
        match c {
            '|' => Kind::Vertical,
            '-' => Kind::Horizontal,
            'L' => Kind::NorthEast,
            'J' => Kind::NorthWest,
            '7' => Kind::SouthWest,
            'F' => Kind::SouthEast,
            '.' => Kind::Ground,
            'S' => Kind::Start,
            _ => panic!("failed to parse tile")
        }
    }

    // right turn: +1
    // left turn: -1
    fn orientation(&self, dir: &Direction) -> i32 {
        match self {
            Kind::Vertical   => 0,
            Kind::Horizontal => 0,
            Kind::NorthEast  => if *dir == Direction::West  {1} else {-1},
            Kind::NorthWest  => if *dir == Direction::South {1} else {-1},
            Kind::SouthWest  => if *dir == Direction::East  {1} else {-1},
            Kind::SouthEast  => if *dir == Direction::North {1} else {-1},
            Kind::Ground     => 0,
            Kind::Start      => 0,
        }
    }
}

#[derive(Debug)]
struct Path {
    links: Vec<(i32, i32, Direction)>,
    oriented_right: bool
}

#[derive(Debug)]
struct Sketch {
    tiles: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
    start: (i32, i32)
}

impl Sketch {
    fn parse(s: &str) -> Sketch {
        let mut tiles = vec![];

        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(Tile::parse(c));
            }
            tiles.push(row);
        }

        let width = tiles[0].len() as i32;
        let height = tiles.len() as i32;
        let mut start = (0 as i32, 0 as i32);
        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.kind == Kind::Start {
                    start = (x as i32, y as i32);
                }
            }
        }

        Sketch {
            tiles,
            width,
            height,
            start
        }
    }

    // can also print the path directions
    // but the directions are not as expected... it works anyway?
    fn print(&self) {
        let mut chars = vec![];
        chars.resize(self.height as usize, vec![]);

        for y in 0..self.height {
            let mut row = vec![];
            row.resize(self.width as usize, '?');
            chars[y as usize] = row;

            for x in 0..self.width {
                chars[y as usize][x as usize] = match self.get(x,y).topology {
                    Topology::Boundary => '.',
                    Topology::Outside  => 'O',
                    Topology::Inside   => 'I',
                    Topology::Unknown  => '?'
                };
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let c = chars[y as usize][x as usize];
                print!("{c}");
            }
            println!();
        }
    }

    fn get(&self, x: i32, y: i32) -> &Tile {
        if !(0..self.width).contains(&x) {
            panic!("invalid get: x={x}");
        }

        if !(0..self.height).contains(&y) {
            panic!("invalid get: y={y}");
        }

        &self.tiles[y as usize][x as usize]
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        if !(0..self.width).contains(&x) {
            panic!("invalid get: x={x}");
        }

        if !(0..self.height).contains(&y) {
            panic!("invalid get: y={y}");
        }

        &mut self.tiles[y as usize][x as usize]
    }

    fn next_loop_direction(&self, tile: Tile, dir: Direction) -> Direction {
        match tile.kind {
            Kind::Vertical   => if dir == Direction::North {Direction::North} else {Direction::South},
            Kind::Horizontal => if dir == Direction::East  {Direction::East}  else {Direction::West},
            Kind::NorthEast  => if dir == Direction::West  {Direction::North} else {Direction::East},
            Kind::NorthWest  => if dir == Direction::East  {Direction::North} else {Direction::West},
            Kind::SouthWest  => if dir == Direction::East  {Direction::South} else {Direction::West},
            Kind::SouthEast  => if dir == Direction::West  {Direction::South} else {Direction::East},
            Kind::Ground     => panic!("invalid next pipe: ground"),
            Kind::Start      => panic!("invalid next pipe: start"),
        }
    }

    fn mark_loop(&mut self) -> Path {
        let mut current;
        let mut dir;
        if self.start.0 > 0 && [Kind::Horizontal, Kind::NorthEast, Kind::SouthEast].contains(&self.get(self.start.0-1, self.start.1).kind) {
            current = (self.start.0-1, self.start.1);
            dir = Direction::West;
        } else if self.start.0 < self.width - 1 && [Kind::Horizontal, Kind::NorthWest, Kind::SouthWest].contains(&self.get(self.start.0+1, self.start.1).kind) {
            current = (self.start.0+1, self.start.1);
            dir = Direction::East;
        } else {
            current = (self.start.0, self.start.1-1);
            dir = Direction::North;
        }

        let mut links = vec![(self.start.0, self.start.1, dir)];
        self.get_mut(self.start.0, self.start.1).topology = Topology::Boundary;
        let mut orientation = 0;
        while current != self.start {
            let tile = self.get_mut(current.0, current.1);
            tile.topology = Topology::Boundary;
            orientation += tile.kind.orientation(&dir);
            let tile = tile.clone();
            links.push((current.0, current.1, dir));

            dir = self.next_loop_direction(tile, dir);
            match dir {
                Direction::North => {current.1 -= 1},
                Direction::South => {current.1 += 1},
                Direction::East =>  {current.0 += 1},
                Direction::West =>  {current.0 -= 1},
            }
        }

        let path = Path {
            links,
            oriented_right: orientation >= 0
        };

        path
    }

    // mark the local topology around a tile: 3 x 3 neighbourhood
    // center has to have topology = Boundary
    fn mark_local(&mut self, x: i32, y: i32, kind: Kind, dir: Direction, oriented_right: bool) {
        if kind == Kind::Start || kind == Kind::Ground  {
            return;
        }

        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                let a = x + dx;
                let b = y + dy;
                if (a,b) == (0,0) || !(0..self.width).contains(&a) || !(0..self.height).contains(&b) {
                    continue;
                }

                let mut inside = match (kind, dir) {
                    (Kind::Vertical, Direction::North)  => dx > 0,
                    (Kind::Vertical, Direction::South)  => dx < 0,

                    (Kind::Horizontal, Direction::West) => dy < 0,
                    (Kind::Horizontal, Direction::East) => dy > 0,

                    (Kind::NorthEast, Direction::South) => (dx, dy) != (1, -1),
                    (Kind::NorthEast, Direction::West)  => (dx, dy) == (1, -1),

                    (Kind::NorthWest, Direction::South) => (dx, dy) == (-1, -1),
                    (Kind::NorthWest, Direction::East)  => (dx, dy) != (-1, -1),

                    (Kind::SouthEast, Direction::North) => (dx, dy) == (1, 1),
                    (Kind::SouthEast, Direction::West)  => (dx, dy) != (1, 1),

                    (Kind::SouthWest, Direction::North) => (dx, dy) != (-1, 1),
                    (Kind::SouthWest, Direction::East)  => (dx, dy) == (-1, 1),

                    _ => false // don't care
                };

                let tile = self.get_mut(a, b);
                if tile.topology == Topology::Unknown {
                    inside ^= !oriented_right;
                    tile.topology = if inside {Topology::Inside} else {Topology::Outside};
                }
            }
        }
    }

    fn mark_small(&mut self, path: &Path) {
        for (x, y, dir) in &path.links {
            let kind = self.get(*x, *y).kind;
            self.mark_local(*x, *y, kind, *dir, path.oriented_right);
        }
    }

    fn mark_big(&mut self) -> i32 {
        let mut counter = 0;

        for y in 0..self.height {
            let mut mode = Topology::Outside;
            for x in 0..self.width {
                let tile = self.get_mut(x, y);

                if tile.topology == Topology::Unknown {
                    tile.topology = mode;
                } else {
                    mode = tile.topology;
                }

                if tile.topology == Topology::Inside {
                    counter += 1;
                }
            }
        }

        counter
    }
}
