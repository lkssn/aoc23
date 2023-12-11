fn main() {
    let input = std::fs::read_to_string("day10/data/example3.txt").expect("failed to read file");
    let sketch = Sketch::parse(input.as_str());
    let path = sketch.find_loop();
    let distance = path.len() / 2;
    println!("distance: {distance}");
}

#[derive(Debug,PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

impl Tile {
    fn parse(c: char) -> Tile {
        match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("failed to parse tile")
        }
    }
}

#[derive(Debug)]
struct Sketch {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: (usize, usize)
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

        let width = tiles[0].len();
        let height = tiles.len();
        let mut start = (0, 0);
        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    start = (x, y);
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

    fn get(&self, x: usize, y: usize) -> &Tile {
        if !(0..self.width).contains(&x) {
            panic!("invalid get: x={x}");
        }

        if !(0..self.height).contains(&y) {
            panic!("invalid get: y={y}");
        }

        &self.tiles[y][x]
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self.get(x, y) {
            Tile::Vertical   => vec![(x, y-1), (x, y+1)],
            Tile::Horizontal => vec![(x-1, y), (x+1, y)],
            Tile::NorthEast  => vec![(x, y-1), (x+1, y)],
            Tile::NorthWest  => vec![(x, y-1), (x-1, y)],
            Tile::SouthWest  => vec![(x, y+1), (x-1, y)],
            Tile::SouthEast  => vec![(x, y+1), (x+1, y)],
            Tile::Ground     => vec![],
            Tile::Start      => vec![],
        }
    }

    fn find_loop(&self) -> Vec<(usize, usize)> {
        let mut prev = self.start;
        let mut current;
        if prev.0 > 0 && [Tile::Horizontal, Tile::NorthEast, Tile::SouthEast].contains(self.get(prev.0-1, prev.1)) {
            current = (prev.0-1, prev.1);
        } else if prev.0 < self.width - 1 && [Tile::Horizontal, Tile::NorthWest, Tile::SouthWest].contains(self.get(prev.0+1, prev.1)) {
            current = (prev.0+1, prev.1);
        } else {
            current = (prev.0, prev.1-1);
        }

        let mut path = vec![current];
        while current != self.start {
            for next in self.get_neighbours(current.0, current.1) {
                if next != prev {
                    prev = current;
                    current = next;
                    path.push(current);
                    break;
                }
            }
        }

        path
    }
}
