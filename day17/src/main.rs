fn main() {
    let input = std::fs::read_to_string("day17/data/small.txt").expect("failed to read file");
    let mut map = Map::parse(&input);
    let s = Point::new(0, 0);
    let t = Point::new(map.w-1, map.h-1);
    let distance = map.find_optimal_heat_loss(s, t);

    map.print_tree();
    println!("distance: {}", distance);
}

struct Tile {
    heat_loss: i64,
    marked: bool,
    distance: i64,
    parent: Option<Point>
}

impl Tile {
    fn new(heat_loss: i64) -> Tile {
        Tile { heat_loss, marked: false, distance: i64::MAX, parent: None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn print(&self) {
        let c = match self {
            Self::North => '^',
            Self::East =>  '>',
            Self::South => 'v',
            Self::West =>  '<',
        };
        print!("{}", c);
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn neighbourhood(&self) -> Vec<Point> {
        vec![
            Point { x: self.x,   y: self.y-1 },
            Point { x: self.x+1, y: self.y   },
            Point { x: self.x,   y: self.y+1 },
            Point { x: self.x-1, y: self.y   },
        ]
    }

    fn get_direction(&self, next: &Self) -> Direction {
        let d = self.x.abs_diff(next.x) + self.y.abs_diff(next.y);
        if d != 1 {
            panic!("get_direction: not neighbours {:?}, {:?}", self, next);
        }

        if self.y > next.y {
            return Direction::North;
        }

        if self.y < next.y {
            return Direction::South;
        }

        if self.x < next.x {
            return Direction::East;
        }

        return Direction::West;
    }
}

struct Map {
    w: i64,
    h: i64,
    tiles: Vec<Tile>
}

impl Map {
    fn parse(s: &str) -> Map {
        let mut map = Map { w: 0, h: 0, tiles: vec![] };

        for line in s.lines() {
            for c in line.chars() {
                let heat_loss = c as i64 - '0' as i64;
                let tile = Tile::new(heat_loss);
                map.tiles.push(tile);
            }
            map.h += 1;
        }
        map.w = map.tiles.len() as i64 / map.h;

        map
    }

    fn print_tree(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                let p = Point::new(x, y);
                let tile = self.get(p);

                if let Some(parent) = tile.parent {
                    let direction = p.get_direction(&parent);
                    direction.print();
                } else {
                    if p == Point::new(0, 0) {
                        print!("s");
                    } else {
                        panic!("unreachable point");
                    }
                }
            }
            println!();
        }
    }

    fn get(&self, p: Point) -> &Tile {
        let index = p.y * self.w + p.x;
        &self.tiles[index as usize]
    }

    fn get_mut(&mut self, p: Point) -> &mut Tile {
        let index = p.y * self.w + p.x;
        &mut self.tiles[index as usize]
    }

    fn remove_minimal_point(&self, v: &mut Vec<Point>) -> Point {
        if v.is_empty() {
            panic!("remove_minimal_point: empty");
        }

        let mut min_index = 0;

        for (i, p) in v.iter().enumerate() {
            let d = self.get(*p).distance;
            let min_d = self.get(v[min_index]).distance;

            if d < min_d {
                min_index = i;
            }
        }

        v.remove(min_index)
    }

    fn find_optimal_heat_loss(&mut self, s: Point, t: Point) -> i64 {
        let mut boundary = vec![];
        self.get_mut(s).distance = 0;
        self.get_mut(s).marked = true;
        boundary.push(s);

        // Solve the single source shortest path problem with Dijkstra's Algorithm.
        // Do a 3-Partition of the set of nodes and iterate:
        // 1. Known: We know the shortest path to these nodes. -> Known = {}
        // 2. Boundary: Candidate nodes for further exploration of the unknown. -> Boundary = {s}
        // 3. Unknown: We have not seen these nodes yet. -> Unknown = V

        // TODO: Extend Dijkstra's Algorithm with the feasible path constraint.
        // Feasible Path Constraint:
        // You can move at most three blocks in a single direction
        // before you have to turn left or right.

        while !boundary.is_empty() {
            // extend shortest path tree with one new node
            let next = self.remove_minimal_point(&mut boundary);
            let next_d = self.get(next).distance;

            // update the boundary
            for neighbour in next.neighbourhood() {
                // check rectangle-bounds for potential neighbours
                if !(0..self.w).contains(&neighbour.x) || !(0..self.h).contains(&neighbour.y) {
                    continue;
                }

                let tile = self.get_mut(neighbour);
                let new_distance = next_d + tile.heat_loss;

                if new_distance < tile.distance {
                    tile.parent = Some(next);
                    tile.distance = new_distance;
                }

                if !tile.marked {
                    tile.marked = true;
                    boundary.push(neighbour);
                }
            }
        }

        self.get(t).distance
    }
}
