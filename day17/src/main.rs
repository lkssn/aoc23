fn main() {
    let input = std::fs::read_to_string("day17/data/example.txt").expect("failed to read file");
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>
}

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
            Point { x: self.x-1, y: self.y   },
            Point { x: self.x+1, y: self.y   },
            Point { x: self.x,   y: self.y-1 },
            Point { x: self.x,   y: self.y+1 },
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
                        print!("R");
                    } else {
                        panic!("unreachable point");
                    }
                }
            }
            println!();
        }
    }

    // Calculate the number of consecutive points in the path p -> ... -> root
    // into a given direction until the first turn or end of path.
    fn calc_consecutive_points(&self, p: Point, dir: Direction) -> i64 {
        let mut counter = 1;

        let delta = match dir {
            Direction::North => Point::new( 0, -1),
            Direction::East  => Point::new( 1,  0),
            Direction::South => Point::new( 0,  1),
            Direction::West  => Point::new(-1,  0),
        };

        let mut current = p;
        loop {
            if let Some(parent) = self.get(current).parent {
                let difference = Point::new(parent.x - current.x, parent.y - current.y);

                if difference == delta {
                    counter += 1;
                    current = parent;
                } else {
                    // end of line
                    break;
                }
            } else {
                // end of path -> also end of line
                break;
            }
        }

        counter
    }

    fn print_path(&self, path: &Path) {
        for y in 0..self.h {
            for x in 0..self.w {
                let p = Point::new(x, y);
                let tile = self.get(p);

                if let Some(parent) = tile.parent {
                    if path.points.contains(&p) {
                        let direction = p.get_direction(&parent);
                        direction.print();
                    } else {
                        print!("{}", '.');
                    }
                } else {
                    print!("{}", '.');
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
            panic!("find_minimal_distance: empty");
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

        // Do a 3-Partition of the set of nodes:
        // 1. Known: We know the shortest path to these nodes. -> Known = {}
        // 2. Boundary: Candidate nodes for further exploration of the unknown. -> Boundary = {s}
        // 3. Unknown: We have not seen these nodes yet. -> Unknown = V

        // TODO: Reduce amount of feasible paths according to the max-3-consecutive-rule.
        // This means maximally 4 nodes in a line.
        // Idea 1: Check before considering any path -> If property violated: Drop this path.
        //         -> Does not work: Sometimes have to backtrack further!
        // Idea 2: Use normal Dijkstra and afterwards modify the shortest path iteratively to make it feasible.

        while !boundary.is_empty() {
            // extend shortest path tree with one new node
            let next = self.remove_minimal_point(&mut boundary);

            // update the boundary
            let neighbourhood = next.neighbourhood();
            let next_d = self.get(next).distance;
            for neighbour in neighbourhood {
                // check rectangle-bounds for potential neighbours
                if !(0..self.w).contains(&neighbour.x) || !(0..self.h).contains(&neighbour.y) {
                    continue;
                }

                // check if the new path is a viable path
                let dir = neighbour.get_direction(&next);
                let counter = 1 + self.calc_consecutive_points(next, dir);
                if counter >= 5 {
                    // 4 nodes in a line are OK (3 edges)
                    // 5 and more nodes: violation of the path property
                    // not a viable path: drop the path
                    println!("dropping not viable path: {:?} -> {:?}", next, neighbour);
                    continue;
                }

                let tile = self.get_mut(neighbour);

                if next_d + tile.heat_loss < tile.distance {
                    tile.parent = Some(next);
                    tile.distance = next_d + tile.heat_loss;
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